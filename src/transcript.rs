use std::fmt;

use crate::position::Position;

pub const MANUBU_MARUO: &str = "E6F4E3F6G5D6E7F5C5";
pub const DEPTH_FIRST: &str = "D3C3B3B2B1A1C4C1C2D2D1E1A2A3F5E2F1G1PPF2PPE3PPB5B4A5A4C5A6F4F3G3G2H2H1H3H4G4C6G5H5B6C7D6E6F6G6H6H7A7PPB7A8D7E7F7G7G8B8C8D8E8F8H8";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transcript {
    // starts a (A, 1) and goes to (H, 8)
    Position(char, usize),
    // indicates the player passed
    Pass,
}

impl fmt::Display for Transcript {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transcript::Position(x, y) => write!(f, "{}{}", x, y),
            Transcript::Pass => write!(f, "PP"),
        }
    }
}

impl Transcript {
    pub fn from_string(source: &str) -> Vec<Self> {
        let mut output = Vec::new();

        let mut chars = source.chars();

        while let Some(raw_x) = chars.next() {
            if let Some(raw_y) = chars.next() {
                output.push(Transcript::from_chars(raw_x, raw_y))
            }
        }

        output
    }

    pub fn from_chars(raw_x: char, raw_y: char) -> Self {
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
            output.push_str(&format!("{}", t));
        }
        output
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;

    #[test]
    fn transcript_round_trip() {
        let transcript_source = String::from("C4E3F4G5G4G3E2C3H6C5B4F3H5B3H3A5A4F2G2B5A6H2B6H1H4E1G1B7D1H7C6A7A2F1B8D3A8F5C2B2A3D6E7C1B1C7D8A1F6D2G6F8D7G7E6C8E8PPF7PPH8G8PP");
        let transcript_vec = Transcript::from_string(&transcript_source);

        // round trip to vector format
        assert_eq!(transcript_source, Transcript::stringify(&transcript_vec));

        // round trip to game
        let tv_to_game = Game::from_transcript(&transcript_vec);
        assert_eq!(transcript_vec, tv_to_game.transcript);
    }
}
