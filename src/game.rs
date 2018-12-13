use crate::board::Board;
use crate::direction::Direction;
use crate::disk::{Disk, DiskIter};
use crate::position::{Position, PositionIter, PositionState};
use crate::transcript::Transcript;

#[derive(Clone, Debug, Ord, PartialOrd, Hash, Eq, PartialEq)]
pub struct ValidMove {
    pub position: Position,
    pub affected: Vec<Position>,
}

#[derive(Debug)]
pub struct ValidMoveIterator {
    disk: Disk,
    board: Board,
    position_iter: PositionIter,
}

impl ValidMoveIterator {
    pub fn new(board: Board, disk: Disk) -> Self {
        ValidMoveIterator {
            disk,
            board,
            position_iter: board.iter(),
        }
    }
}

impl Iterator for ValidMoveIterator {
    type Item = ValidMove;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // work through the position iterator until we find
            // either a valid move, or we're out of available positions.
            match self.position_iter.next() {
                Some(position) => {
                    // given a position, we try to determine if it's a valid
                    // move; if it isn't, continue on to the next available position
                    match Game::validate_move(&self.board, &position, self.disk) {
                        Some(valid_move) => return Some(valid_move),
                        None => continue,
                    }
                }
                // Ahh, no more positions! Time to bail out.
                None => return None,
            }
        }
    }
}

#[derive(Clone)]
pub struct Game {
    pub transcript: Vec<Transcript>,
    turn: Disk,
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::empty();

        board.set(3, 3, PositionState::Light);
        board.set(4, 3, PositionState::Dark);
        board.set(3, 4, PositionState::Dark);
        board.set(4, 4, PositionState::Light);

        Game {
            transcript: Vec::new(),
            turn: Disk::Dark,
            board,
        }
    }

    pub fn from_transcript(transcript: &[Transcript]) -> Self {
        let mut game = Game::new();

        for t in transcript {
            game = match t.to_position() {
                None => game.pass(),
                Some(position) => {
                    if let Some(valid_move) = Game::validate_move(&game.board, &position, game.turn)
                    {
                        game.play(valid_move)
                    } else {
                        panic!(
                            "Invalid move {:?} for board {:?} ({})",
                            position, game.board, game.turn
                        )
                    }
                }
            }
        }
        game
    }

    pub fn play(&self, vm: ValidMove) -> Self {
        // Operate on a copy, keeping self immutable.
        let mut g = self.clone();

        g.board.set(vm.position.x, vm.position.y, g.turn.into());

        for flip in vm.affected {
            g.board.flip(flip.x, flip.y);
        }

        g.transcript.push(vm.position.into());
        Game::end_turn(g)
    }

    pub fn pass(&self) -> Self {
        // Operate on a copy, keeping self immutable.
        let mut g = self.clone();
        g.transcript.push(Transcript::Pass);
        Game::end_turn(g)
    }

    // Determines whether a grid can be played by either Light or Dark
    pub fn is_complete(&self) -> bool {
        for p in PositionIter::new() {
            for d in DiskIter::new() {
                if Game::validate_move(&self.board, &p, d).is_some() {
                    return false;
                }
            }
        }
        true
    }

    pub fn score(&self) -> (usize, usize) {
        let mut dark_score = 0;
        let mut light_score = 0;
        for p in self.board.iter() {
            match self.board.get(p.x, p.y) {
                PositionState::Empty => continue,
                PositionState::Dark => dark_score += 1,
                PositionState::Light => light_score += 1,
            }
        }
        (dark_score, light_score)
    }

    pub fn move_iter(&self) -> ValidMoveIterator {
        ValidMoveIterator::new(self.board, self.turn)
    }

    pub fn valid_moves(&self) -> Vec<ValidMove> {
        self.move_iter().map(|m| m.clone()).collect()
    }

    pub fn pp(&self) {
        let (dark_score, light_score) = self.score();
        let next_turn = if self.is_complete() {
            "Complete".to_string()
        } else {
            format!("{}", self.turn)
        };

        println!("{}", self.board.to_string());
        println!("Transcript: {}", Transcript::stringify(&self.transcript));
        println!("Score: Dark {}, Light {}", dark_score, light_score);
        println!("Next turn: {}", next_turn);
    }

    fn end_turn(mut game: Game) -> Self {
        game.turn = game.turn.opposite();
        game
    }

    // Determines whether a given position can be played, and what it's effect will be.
    fn validate_move(board: &Board, position: &Position, disk: Disk) -> Option<ValidMove> {
        // println!("validate_move {:?} for {}", position, disk);
        // println!("{}", board);

        // before anything else, the position must be empty.
        if board.get(position.x, position.y) != PositionState::Empty {
            // println!("Position isn't empty!");
            return None;
        }

        // save this for comparisons later on.
        let disk_state: PositionState = disk.into();

        // where we collect potentially flippable pieces
        let mut flippable = Vec::new();

        for direction in &Direction::ALL {
            // recenter ourselves and clear our potential flippers before exploring in new directions
            let mut current_position = *position;
            let mut maybe_flippable = Vec::new();

            // loop through neighbors to see if we can find anything
            loop {
                match current_position.neighbor(direction) {
                    None => break, // no neighbor in that direction, move on to the next one
                    Some(new_position) => {
                        let np_state = board.get(new_position.x, new_position.y);

                        if np_state == PositionState::Empty {
                            // womp womp, no luck, break and find a new direction!
                            break;
                        }

                        if np_state != disk_state {
                            // found a potential flipper; move on to check the next one!
                            maybe_flippable.push(new_position);
                            current_position = new_position;
                            continue;
                        }

                        if np_state == disk_state {
                            // found a potential end; add any flippables we found, and
                            // move on to the next direction.
                            flippable.append(&mut maybe_flippable);
                            break;
                        }
                    }
                }
            }
        }

        // done checking our directions. If we have any flippables, return 'em!
        if flippable.is_empty() {
            None
        } else {
            Some(ValidMove {
                position: *position,
                affected: flippable,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcript::{DEPTH_FIRST, MANUBU_MARUO};
    use std::mem;

    #[test]
    fn validate_positions() {
        let g = Game::new();

        // yes, we have opening moves
        assert_eq!(g.is_complete(), false);

        // three spots that are invalid for either player, an empty spot and two occupied spots of different colors
        let invalid_moves = vec![
            Position { x: 0, y: 0 },
            Position { x: 3, y: 3 },
            Position { x: 3, y: 4 },
        ];
        for m in invalid_moves {
            let mut result = Game::validate_move(&g.board, &m, Disk::Dark);
            assert_eq!(result, None);
            result = Game::validate_move(&g.board, &m, Disk::Light);
            assert_eq!(result, None);
        }

        // check a spot that's valid for one color, but not the other
        let p = Position { x: 3, y: 2 }; // good for dark, not light
        let mut result = Game::validate_move(&g.board, &p, Disk::Dark);
        assert!(result.is_some());
        result = Game::validate_move(&g.board, &p, Disk::Light);
        assert!(result.is_none());
    }

    #[test]
    fn vmi_finds_all_moves() {
        let g = Game::new();

        assert_eq!(g.valid_moves().len(), 4);
    }

    #[test]
    fn completion() {
        let mut g = Game::new();
        assert_eq!(g.is_complete(), false);

        g = Game::from_transcript(&Transcript::from_string(MANUBU_MARUO));
        assert_eq!(g.is_complete(), true);
    }

    #[test]
    fn score() {
        let mut g = Game::new();
        let (dark_score, light_score) = g.score();
        assert_eq!(dark_score, light_score);
        assert_eq!(dark_score, 2);

        g = Game::from_transcript(&Transcript::from_string(MANUBU_MARUO));
        let (dark_score, light_score) = g.score();
        assert_eq!(dark_score, 13);
        assert_eq!(light_score, 0);
    }

    #[test]
    fn mem_size() {
        let g = Game::from_transcript(&Transcript::from_string(DEPTH_FIRST));
        println!("Game mem size: {}", mem::size_of_val(&g));
    }

}
