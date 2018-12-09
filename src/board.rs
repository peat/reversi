use crate::direction::Direction;
use crate::internal_position::InternalPosition;
use crate::position_state::PositionState;
use crate::transcript_position::TranscriptPosition;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Board {
    pub moves: Vec<InternalPosition>,
    pub turn: PositionState,
    pub passed: bool, // whether the last player passed
    board: [[PositionState; Board::MAX_Y + 1]; Board::MAX_X + 1],
}

impl Default for Board {
    // creates a new board, with starting positions filled and the first move
    // given to the Dark player.
    fn default() -> Self {
        let mut b = Board {
            moves: Vec::new(),
            turn: PositionState::Dark,
            passed: false,
            board: [[PositionState::Empty; 8]; 8],
        };

        b.set(&InternalPosition { x: 4, y: 3 }, PositionState::Dark);
        b.set(&InternalPosition { x: 3, y: 3 }, PositionState::Light);
        b.set(&InternalPosition { x: 3, y: 4 }, PositionState::Dark);
        b.set(&InternalPosition { x: 4, y: 4 }, PositionState::Light);

        b // clean board in starting position!
    }
}

impl Board {
    pub const MAX_X: usize = 7;
    pub const MAX_Y: usize = 7;

    fn get(&self, p: &InternalPosition) -> PositionState {
        self.board[p.x][p.y]
    }

    pub fn attempt_direction(
        &self,
        position: InternalPosition,
        player_state: PositionState,
        direction: &Direction,
    ) -> Option<Vec<InternalPosition>> {
        let mut found_positions = Vec::new();

        // placeholder while we're traversing the board
        let mut current_position = position;

        loop {
            // see if we can load the next position in the given direction
            match current_position.direction(direction) {
                // whoops, hit an edge; nothing in this direction!
                None => return None,

                // ok, we found a neighbor. let's check it out ...
                Some(neighbor) => {
                    match self.get(&neighbor) {
                        // Aww dang, we hit an empty space; abort!
                        PositionState::Empty => return None,
                        s @ PositionState::Dark | s @ PositionState::Light => {
                            // if our neighbor is in the same state as the player,
                            // it MIGHT mean we've been collecting flippable positions!
                            if s == player_state {
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

    pub fn moves_for(
        &self,
        player_state: PositionState,
    ) -> HashMap<InternalPosition, Vec<InternalPosition>> {
        // placeholder for our playable set
        let mut playable_set = HashMap::new();

        // gather all of the empty, playable spaces
        let empty_positions = self.in_state(PositionState::Empty);

        // step through each empty position and determine if it is playable.
        for position in empty_positions {
            // we'll collect all of the positions that would be flipped by a play at the
            // current position here
            let mut affected_positions = Vec::new();

            // cycle through our cardinal directions
            for direction in &Direction::ALL {
                // if we find a play, collect the potentially affected positions
                if let Some(affected) = self.attempt_direction(position, player_state, direction) {
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

    pub fn in_state(&self, s: PositionState) -> Vec<InternalPosition> {
        let mut output = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                if self.board[x][y] == s {
                    output.push(InternalPosition { x, y })
                }
            }
        }

        output
    }

    fn set(&mut self, p: &InternalPosition, disk: PositionState) {
        self.board[p.x][p.y] = disk;
    }

    pub fn play(&mut self, t: TranscriptPosition) {
        let available_moves = self.moves_for(self.turn);
        let internal_point: InternalPosition = t.into();

        match available_moves.get(&internal_point) {
            None => return,
            Some(affected) => {
                // mark the position as owned by the current player
                self.set(&internal_point, self.turn);

                // flip 'em
                for a in affected {
                    self.flip(a)
                }

                // record the move
                self.moves.push(internal_point);

                // on to the next turn!
                self.passed = false;
                self.turn = PositionState::opposite(self.turn);
            }
        }
    }

    pub fn pass(&mut self) {
        self.turn = PositionState::opposite(self.turn);
        self.passed = true;
    }

    fn flip(&mut self, p: &InternalPosition) {
        self.set(p, PositionState::opposite(self.get(p)));
    }

    pub fn pp(&self) {
        println!("  a b c d e f g h");
        for y in 0..8 {
            print!("{}", y + 1);
            for x in 0..8 {
                match self.board[x][y] {
                    PositionState::Empty => print!(" •"),
                    PositionState::Dark => print!(" D"),
                    PositionState::Light => print!(" L"),
                }
            }
            println!();
        }
    }

    pub fn fmt_points(ps: Vec<InternalPosition>) -> String {
        let mut tmp = Vec::new();
        for p in ps {
            let t: TranscriptPosition = p.into();
            tmp.push(t.format())
        }
        tmp.join(", ")
    }
}
