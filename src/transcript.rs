use std::fmt;

use crate::position::Position;

pub const MANUBU_MARUO: &str = "E6F4E3F6G5D6E7F5C5";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transcript {
    Play(Position),
    Pass,
}

impl fmt::Display for Transcript {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Transcript::Play(p) => {
                let x_char = Transcript::x_to_char(p.x);
                let y_out = p.y + 1;
                write!(f, "{}{}", x_char, y_out)
            }
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
                let t = Transcript::from_chars(raw_x, raw_y);
                output.push(t)
            }
        }

        output
    }

    pub fn from_chars(raw_x: char, raw_y: char) -> Self {
        let x = match raw_x.to_ascii_uppercase() {
            'P' => return Transcript::Pass,
            c => Transcript::char_to_x(c),
        };

        let y = Transcript::char_to_y(raw_y);

        let position = Position { x, y };
        Transcript::Play(position)
    }

    pub fn stringify(transcripts: &[Transcript]) -> String {
        let mut output = String::new();
        for t in transcripts {
            output.push_str(&format!("{}", t));
        }
        output
    }

    pub fn symmetrical(original: Vec<Transcript>) -> Vec<Vec<Transcript>> {
        // first position is rotated.
        let first = Transcript::transform(original.clone(), Position::rotate);
        // second position is flipped.
        let second = Transcript::transform(original.clone(), Position::flip);
        // third position is rotated and flipped.
        let third = Transcript::transform(first.clone(), Position::flip);

        vec![first, second, third, original]
    }

    fn transform(transcripts: Vec<Transcript>, f: fn(Position) -> Position) -> Vec<Self> {
        let mut output = Vec::new();
        for t in transcripts {
            let new_t = match t {
                Transcript::Pass => Transcript::Pass,
                Transcript::Play(p) => Transcript::Play(f(p)),
            };
            output.push(new_t);
        }

        output
    }

    fn char_to_x(c: char) -> usize {
        match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => panic!("Invalid X {}; must be A..H", c),
        }
    }

    fn char_to_y(c: char) -> usize {
        match c.to_digit(10) {
            None => panic!("Invalid Y {}; must be 1..8", c),
            Some(y) => (y - 1) as usize,
        }
    }

    fn x_to_char(x: usize) -> char {
        match x {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            4 => 'E',
            5 => 'F',
            6 => 'G',
            7 => 'H',
            _ => panic!("Invalid X {}; must be 0..7", x),
        }
    }
}

impl From<Position> for Transcript {
    fn from(p: Position) -> Self {
        Transcript::Play(p)
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
        let tv_to_game = Game::from_transcript(transcript_vec.clone());
        assert_eq!(transcript_vec, tv_to_game.transcript);
    }

    #[test]
    fn transform_round_trip() {
        let transcript_source = String::from("C4E3F4G5G4G3E2C3H6C5B4F3H5B3H3A5A4F2G2B5A6H2B6H1H4E1G1B7D1H7C6A7A2F1B8D3A8F5C2B2A3D6E7C1B1C7D8A1F6D2G6F8D7G7E6C8E8PPF7PPH8G8PP");
        let transcript_vec = Transcript::from_string(&transcript_source);

        // two flips should return to original
        let flip_one = Transcript::transform(transcript_vec.clone(), Position::flip);
        let flip_two = Transcript::transform(flip_one.clone(), Position::flip);
        assert_eq!(transcript_vec, flip_two);

        // two rotates should return to original
        let rotate_one = Transcript::transform(transcript_vec.clone(), Position::rotate);
        let rotate_two = Transcript::transform(rotate_one.clone(), Position::rotate);
        assert_eq!(transcript_vec, rotate_two);
    }
}
