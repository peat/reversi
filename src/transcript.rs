use crate::position::Position;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transcript {
    // starts a (A, 1) and goes to (H, 8)
    Position(char, usize),
    // indicates the player passed
    Pass,
}

impl Transcript {
    pub fn from_string(source: &str) -> Vec<Transcript> {
        let mut output = Vec::new();

        let mut chars = source.chars();

        while let Some(raw_x) = chars.next() {
            if let Some(raw_y) = chars.next() {
                output.push(Transcript::from_chars(raw_x, raw_y))
            }
        }

        output
    }

    pub fn from_chars(raw_x: char, raw_y: char) -> Transcript {
        match raw_x.to_ascii_uppercase() {
            'P' => Transcript::Pass,
            x => match raw_y.to_digit(10) {
                // base 10
                None => panic!(
                    "Error converting x {:?} y {:?} to Transcript.",
                    raw_x, raw_y
                ),
                Some(y) => Transcript::Position(x, y as usize),
            },
        }
    }

    pub fn stringify(transcripts: &[Transcript]) -> String {
        let mut output = String::new();
        for t in transcripts {
            output.push_str(&t.format());
        }
        output
    }

    pub fn format(&self) -> String {
        match self {
            Transcript::Position(x, y) => format!("{}{}", x, y),
            Transcript::Pass => "PP".to_string(),
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
                    _ => return None,
                };

                if *y < 1 || *y > 8 {
                    return None;
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
