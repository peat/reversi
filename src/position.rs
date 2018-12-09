use crate::board::Board;
use crate::direction::Direction;
use crate::disk::Disk;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Empty,
    Occupied(Disk),
}

impl State {
    pub fn opposite(b: State) -> State {
        match b {
            State::Empty => State::Empty,
            State::Occupied(d) => State::Occupied(d.opposite()),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State::Empty
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn direction(&self, d: &Direction) -> Option<Self> {
        match d {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }

    fn north(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Position {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn south(&self) -> Option<Self> {
        if self.y == Board::MAX_Y {
            None
        } else {
            Some(Position {
                x: self.x,
                y: self.y + 1,
            })
        }
    }

    fn east(&self) -> Option<Self> {
        if self.x == Board::MAX_X {
            None
        } else {
            Some(Position {
                x: self.x + 1,
                y: self.y,
            })
        }
    }

    fn west(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(Position {
                x: self.x - 1,
                y: self.y,
            })
        }
    }
}
