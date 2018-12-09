use crate::position::Position;

// starts a (A, 1) and goes to (H, 8)
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transcript {
    Position(char, usize),
    Pass,
}

impl Transcript {
    pub fn format_vec(transcripts: &Vec<Transcript>) -> String {
        let mut output = String::new();
        for t in transcripts {
            output.push_str(&t.format());
        }
        output
    }

    pub fn format(&self) -> String {
        match self {
            Transcript::Position(x, y) => format!("{}{}", x, y),
            Transcript::Pass => format!("PP"),
        }
    }

    pub fn to_position(&self) -> Option<Position> {
        match self {
            Transcript::Pass => None,
            Transcript::Position(x, y) => {
                let px = match x {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    'E' => 4,
                    'F' => 5,
                    'G' => 6,
                    'H' => 7,
                    _ => return None, // out of bounds
                };

                if *y < 1 || *y > 8 {
                    return None; // out of bounds
                };

                Some(Position { x: px, y: y - 1 })
            }
        }
    }
}

impl From<Position> for Transcript {
    fn from(i: Position) -> Self {
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

        Transcript::Position(x, i.y + 1)
    }
}
