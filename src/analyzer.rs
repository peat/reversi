use crate::direction::Direction;
use crate::disk::Disk;
use crate::grid::{Grid, GridIterator, State};
use crate::position::Position;

#[derive(Clone, Debug, Ord, PartialOrd, Hash, Eq, PartialEq)]
pub struct ValidMove {
    pub position: Position,
    pub affected: Vec<Position>,
}

#[derive(Debug)]
pub struct MoveIterator {
    grid_iterator: GridIterator,
}

impl MoveIterator {
    pub fn new(grid_iterator: GridIterator) -> Self {
        MoveIterator { grid_iterator }
    }
}

impl Iterator for MoveIterator {
    type Item = ValidMove;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.grid_iterator.next() {
                Some(position) => {
                    match analyzer::moves_for(
                        &self.grid_iterator.grid,
                        &position,
                        &self.grid_iterator.grid.turn,
                    ) {
                        Some(valid_move) => return Some(valid_move),
                        None => continue,
                    }
                }
                None => break,
            }
        }
        None
    }
}

mod analyzer {
    use super::*;

    pub fn moves_for(grid: &Grid, position: &Position, disk: &Disk) -> Option<ValidMove> {
        // before anything else, the position must be empty.
        if grid.get(position.x, position.y) != State::Empty {
            return None;
        }

        // save this for comparisons later on.
        let disk_state: State = disk.clone().into();

        // where we collect potentially flippable pieces
        let mut flippable = Vec::new();

        for direction in &Direction::ALL {
            // println!("Attempting direction {:?} ...", direction);

            // recenter ourselves and clear our potential flippers before exploring in new directions
            let mut current_position = position.clone();
            let mut maybe_flippable = Vec::new();

            // loop through neighbors to see if we can find anything
            loop {
                // std::thread::sleep_ms(250);
                // println!("  Current Position: {:?}", current_position);
                match current_position.neighbor(direction) {
                    None => break, // no neighbor in that direction, move on to the next one
                    Some(new_position) => {
                        // println!("  .. examining neighbor {:?}", new_position);
                        let np_state = grid.get(new_position.x, new_position.y);

                        // println!(
                        // "  .. neighbor vs disk_state! {:?} vs {:?}",
                        //     np_state, disk_state
                        // );

                        if np_state == State::Empty {
                            // womp womp, no luck, break and find a new direction!
                            // println!("  ... womp womp ...");
                            break;
                        }

                        if np_state != disk_state {
                            // println!(" ... possible flipper!");
                            // found a potential flipper; move on to check the next one!
                            maybe_flippable.push(new_position);
                            current_position = new_position;
                            continue;
                        }

                        if np_state == disk_state {
                            // println!(" ... ending direction! Found: {:?}", maybe_flippable);
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
                position: position.clone(),
                affected: flippable,
            })
        }
    }
}
