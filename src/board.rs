use crate::direction::Direction;
use crate::disk::Disk;
use crate::position::{Position, State};
use crate::transcript::Transcript;

use std::collections::HashMap;

type MoveMap = HashMap<Position, Vec<Position>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub transcript: Vec<Transcript>, // the history of plays on this board
    pub turn: Disk,                  // who is currently playing
    pub passed: bool,                // whether the last player passed
    pub available_moves: MoveMap,    // the moves available to the current player
    board: [[State; Board::MAX_Y + 1]; Board::MAX_X + 1],
}

impl Default for Board {
    // creates a new board, with starting positions filled and the first move
    // given to the Dark player.
    fn default() -> Self {
        let mut b = Board {
            transcript: Vec::new(),
            turn: Disk::Dark,
            passed: false,
            available_moves: HashMap::new(),
            board: [[State::Empty; 8]; 8],
        };

        // set up the opening positions
        b = Board::set(&b, &Position::new(4, 3), State::Occupied(Disk::Dark));
        b = Board::set(&b, &Position::new(3, 3), State::Occupied(Disk::Light));
        b = Board::set(&b, &Position::new(3, 4), State::Occupied(Disk::Dark));
        b = Board::set(&b, &Position::new(4, 4), State::Occupied(Disk::Light));

        // populate the opening available moves
        b.available_moves = Board::moves_for(&b, b.turn);

        b // clean board in starting position!
    }
}

impl Board {
    pub const MAX_X: usize = 7;
    pub const MAX_Y: usize = 7;

    // GAME PLAY METHODS ------------------------------------------------------

    pub fn play(board: &Board, position: &Position) -> Self {
        let mut new_board = board.clone();

        let affected = match new_board.available_moves.get(&position) {
            None => return new_board,
            Some(affected) => affected,
        };

        // flip 'em
        for a in affected.clone() {
            new_board = Board::flip(&new_board, &a);
        }

        // mark the position as owned by the current player
        new_board = Board::set(&new_board, &position, State::Occupied(new_board.turn));

        // record the move
        new_board.transcript.push(Transcript::from(*position));

        // on to the next turn!
        new_board.passed = false;
        Board::next_turn(&new_board)
    }

    pub fn pass(board: &Board) -> Self {
        let mut new_board = board.clone();
        new_board.passed = true;
        new_board.transcript.push(Transcript::Pass);
        Board::next_turn(&new_board)
    }

    pub fn score(board: &Board, disk: Disk) -> usize {
        Board::in_state(board, State::Occupied(disk)).len()
    }

    pub fn from_transcript(transcript: &[Transcript]) -> Board {
        let mut b = Board::default();

        let plays: Vec<Option<Position>> = transcript.iter().map(|t| t.to_position()).collect();

        for op in plays {
            b = match op {
                Some(p) => Board::play(&b, &p),
                None => Board::pass(&b),
            }
        }

        b
    }

    // INSPECTION METHODS -----------------------------------------------------

    pub fn pp(&self) {
        println!("  a b c d e f g h");
        for y in 0..8 {
            print!("{}", y + 1);
            for x in 0..8 {
                match self.board[x][y] {
                    State::Empty => print!(" •"),
                    State::Occupied(Disk::Dark) => print!(" D"),
                    State::Occupied(Disk::Light) => print!(" L"),
                }
            }
            println!();
        }
    }

    // PRIVATE METHODS --------------------------------------------------------

    fn get(board: &Board, position: &Position) -> State {
        board.board[position.x][position.y]
    }

    fn set(board: &Board, position: &Position, state: State) -> Board {
        let mut new_board = board.clone();
        new_board.board[position.x][position.y] = state;
        new_board
    }

    fn flip(board: &Board, position: &Position) -> Board {
        let old_state = Board::get(&board, position);
        let new_state = State::opposite(old_state);
        Board::set(board, position, new_state)
    }

    fn next_turn(board: &Board) -> Board {
        let mut new_board = board.clone();
        new_board.turn = board.turn.opposite();
        new_board.available_moves = Board::moves_for(&board, new_board.turn);
        new_board
    }

    fn attempt_direction(
        board: &Board,
        position: &Position,
        player_disk: Disk,
        direction: &Direction,
    ) -> Option<Vec<Position>> {
        let mut found_positions = Vec::new();

        // placeholder while we're traversing the board
        let mut current_position = *position;

        loop {
            // see if we can load the next position in the given direction
            match current_position.direction(direction) {
                // whoops, hit an edge; nothing in this direction!
                None => return None,

                // ok, we found a neighbor. let's check it out ...
                Some(neighbor) => {
                    match Board::get(board, &neighbor) {
                        // Aww dang, we hit an empty space; abort!
                        State::Empty => return None,
                        State::Occupied(d) => {
                            // if our neighbor is in the same state as the player,
                            // it MIGHT mean we've been collecting flippable positions!
                            if d == player_disk {
                                // if we've found flippable positions, return 'em, otherwise
                                // return nothing.
                                if found_positions.is_empty() {
                                    return None;
                                } else {
                                    return Some(found_positions);
                                }
                            } else {
                                // if our neighbor is in an opposing state, it it could be a flippable
                                // position! Collect it, and step into the neighbor position.
                                found_positions.push(neighbor);
                                current_position = neighbor;
                            }
                        }
                    }
                }
            }
        }
    }

    fn moves_for(board: &Board, disk: Disk) -> MoveMap {
        // placeholder for our playable set
        let mut playable_set = HashMap::new();

        // gather all of the empty, playable spaces
        let empty_positions = Board::in_state(board, State::Empty);

        // step through each empty position and determine if it is playable.
        for position in empty_positions {
            // we'll collect all of the positions that would be flipped by a play at the
            // current position here
            let mut affected_positions = Vec::new();

            // cycle through our cardinal directions
            for direction in &Direction::ALL {
                // if we find a play, collect the potentially affected positions
                if let Some(affected) = Board::attempt_direction(board, &position, disk, direction)
                {
                    affected_positions.extend(affected);
                }
            }

            // done testing all of the directions; if we have anything, stash it and move on ...
            if !affected_positions.is_empty() {
                playable_set.insert(position, affected_positions);
            }
        }

        playable_set
    }

    fn in_state(board: &Board, s: State) -> Vec<Position> {
        let mut output = Vec::new();

        for y in 0..=Board::MAX_Y {
            for x in 0..=Board::MAX_X {
                if board.board[x][y] == s {
                    output.push(Position { x, y })
                }
            }
        }

        output
    }
}
