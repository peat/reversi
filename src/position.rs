use crate::board::{MAX_X, MAX_Y};
use crate::direction::Direction;
use crate::disk::Disk;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PositionState {
    Empty,
    Dark,
    Light,
}

impl PositionState {
    pub fn opposite(self) -> Self {
        match self {
            PositionState::Empty => PositionState::Empty,
            PositionState::Dark => PositionState::Light,
            PositionState::Light => PositionState::Dark,
        }
    }
}

impl From<Disk> for PositionState {
    fn from(disk: Disk) -> Self {
        match disk {
            Disk::Dark => PositionState::Dark,
            Disk::Light => PositionState::Light,
        }
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Hash, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn neighbor(&self, d: &Direction) -> Option<Self> {
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
        if self.y == MAX_Y {
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
        if self.x == MAX_X {
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

#[derive(Debug)]
pub struct PositionIter {
    index: usize,
}

impl PositionIter {
    pub fn new() -> Self {
        PositionIter { index: 0 }
    }
}

impl Iterator for PositionIter {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        // check to see if our index is out of bounds.
        if self.index >= (MAX_X + 1) * (MAX_Y + 1) {
            return None;
        }

        // convert index into X and Y coordinates.
        let x = self.index % (MAX_X + 1);
        let y = self.index / (MAX_Y + 1);

        // build the response.
        let position = Position { x, y };

        // increment the index
        self.index += 1;

        // result!
        Some(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_iterator() {
        let mut p = PositionIter::new();
        assert_eq!(p.next(), Some(Position { x: 0, y: 0 }));
        assert_eq!(p.last(), Some(Position { x: 7, y: 7 }));

        p = PositionIter::new();
        assert_eq!(p.nth(63), Some(Position { x: 7, y: 7 }));
        assert_eq!(p.next(), None);
        assert_eq!(p.nth(200), None);
    }

}
