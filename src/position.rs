use crate::board::Board;
use crate::direction::Direction;

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
            Direction::NorthEast => self.north_east(),
            Direction::East => self.east(),
            Direction::SouthEast => self.south_east(),
            Direction::South => self.south(),
            Direction::SouthWest => self.south_west(),
            Direction::West => self.west(),
            Direction::NorthWest => self.north_west(),
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

    fn north_east(&self) -> Option<Self> {
        self.north()?.east()
    }

    fn north_west(&self) -> Option<Self> {
        self.north()?.west()
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

    fn south_east(&self) -> Option<Self> {
        self.south()?.east()
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

    fn south_west(&self) -> Option<Self> {
        self.south()?.west()
    }
}
