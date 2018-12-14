use crate::position::{PositionIter, PositionState};

pub const MAX_X: usize = 7;
pub const MAX_Y: usize = 7;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Board {
    data: [[PositionState; MAX_Y + 1]; MAX_X + 1],
}

impl Board {
    pub fn empty() -> Self {
        Board {
            data: [[PositionState::Empty; MAX_Y + 1]; MAX_X + 1],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> PositionState {
        self.data[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, ps: PositionState) {
        self.data[x][y] = ps;
    }

    pub fn flip(&mut self, x: usize, y: usize) -> PositionState {
        let s = self.get(x, y).opposite();
        self.set(x, y, s);
        s
    }

    // returns an iterator of Positions; not strictly necessary, just here for convenience.
    pub fn iter(&self) -> PositionIter {
        PositionIter::new()
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();
        output.push_str("  a b c d e f g h");
        for y in 0..=MAX_Y {
            output.push_str(&format!("\n{}", y + 1));
            for x in 0..=MAX_X {
                match self.get(x, y) {
                    PositionState::Empty => output.push_str(" •"),
                    PositionState::Dark => output.push_str(" D"),
                    PositionState::Light => output.push_str(" L"),
                };
            }
        }
        output.push_str("\n");
        output
    }

    // a board can be rotated 180 degrees and be the same game
    pub fn to_rotated(&self) -> Self {
        let mut positions = self.iter();
        let mut new_board = Board::empty();
        loop {
            match positions.next() {
                None => return new_board,
                Some(p) => {
                    let state = self.get(p.x, p.y);
                    let rp = p.to_rotated();
                    new_board.set(rp.x, rp.y, state);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set_flip() {
        let mut b = Board::empty();

        // check to see if the board is completely empty
        for p in b.iter() {
            assert_eq!(b.get(p.x, p.y), PositionState::Empty);
        }

        // check to see if we can set a particular position
        b.set(0, 0, PositionState::Dark);
        assert_eq!(b.get(0, 0), PositionState::Dark);

        // check to see if we can flip the colors back and forth
        b.flip(0, 0);
        assert_eq!(b.get(0, 0), PositionState::Light);
        b.flip(0, 0);
        assert_eq!(b.get(0, 0), PositionState::Dark);

        // check to see if flipping Empty keeps it as Empty
        b.flip(0, 1);
        assert_eq!(b.get(0, 1), PositionState::Empty);
    }

}
