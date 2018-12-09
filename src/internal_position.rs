use crate::board::Board;
use crate::direction::Direction;
use crate::transcript_position::TranscriptPosition;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct InternalPosition {
    pub x: usize,
    pub y: usize,
}

impl InternalPosition {
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
            Some(InternalPosition {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    fn south(&self) -> Option<Self> {
        if self.y == Board::MAX_Y {
            None
        } else {
            Some(InternalPosition {
                x: self.x,
                y: self.y + 1,
            })
        }
    }

    fn east(&self) -> Option<Self> {
        if self.x == Board::MAX_X {
            None
        } else {
            Some(InternalPosition {
                x: self.x + 1,
                y: self.y,
            })
        }
    }

    fn west(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            Some(InternalPosition {
                x: self.x - 1,
                y: self.y,
            })
        }
    }
}

impl From<TranscriptPosition> for InternalPosition {
    fn from(t: TranscriptPosition) -> Self {
        let x = match t.x {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => panic!("X value out of bounds in: {:?}", t),
        };

        if t.y < 1 || t.y > 8 {
            panic!("Y value out of bounds in: {:?}", t)
        };

        InternalPosition { x, y: t.y - 1 }
    }
}
