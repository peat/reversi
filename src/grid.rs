use crate::analyzer::MoveIterator;
use crate::disk::Disk;
use crate::position::Position;

pub const MAX_X: usize = 7;
pub const MAX_Y: usize = 7;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Empty,
    Dark,
    Light,
}

impl State {
    pub fn opposite(&self) -> State {
        match self {
            State::Empty => State::Empty,
            State::Dark => State::Light,
            State::Light => State::Dark,
        }
    }
}

impl From<Disk> for State {
    fn from(disk: Disk) -> Self {
        match disk {
            Disk::Dark => State::Dark,
            Disk::Light => State::Light,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub turn: Disk,
    data: [[State; MAX_Y + 1]; MAX_X + 1],
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            turn: Disk::Dark,
            data: [[State::Empty; MAX_Y + 1]; MAX_X + 1],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> State {
        self.data[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, state: State) {
        self.data[x][y] = state;
    }

    pub fn flip(&mut self, x: usize, y: usize) -> State {
        let s = self.get(x, y).opposite();
        self.set(x, y, s);
        s
    }

    pub fn iter(&self) -> GridIterator {
        GridIterator::new(self)
    }

    pub fn moves(&self) -> MoveIterator {
        MoveIterator::new(GridIterator::new(self))
    }
}

#[derive(Debug)]
pub struct GridIterator {
    index: usize,
    pub grid: Grid,
}

impl GridIterator {
    pub fn new(grid: &Grid) -> Self {
        // println!("NEW GRID ITERATOR");
        GridIterator {
            index: 0,
            grid: grid.clone(),
        }
    }
}

impl Iterator for GridIterator {
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
