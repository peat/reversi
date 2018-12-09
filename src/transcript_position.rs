use crate::internal_position::InternalPosition;

// starts a (A, 1) and goes to (H, 8)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TranscriptPosition {
    pub x: char,
    pub y: usize,
}

impl TranscriptPosition {
    pub fn format(&self) -> String {
        format!("{}{}", self.x, self.y)
    }
}

impl From<InternalPosition> for TranscriptPosition {
    fn from(i: InternalPosition) -> Self {
        let x = match i.x {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => panic!("X value out of bounds in: {:?}", i),
        };

        if i.y > 7 {
            panic!("Y value out of bounds in: {:?}", i)
        }

        TranscriptPosition { x, y: i.y + 1 }
    }
}
