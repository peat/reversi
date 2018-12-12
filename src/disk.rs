use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Disk {
    Dark,
    Light,
}

impl Disk {
    pub fn opposite(self) -> Self {
        match self {
            Disk::Dark => Disk::Light,
            Disk::Light => Disk::Dark,
        }
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Disk::Dark => write!(f, "Dark"),
            Disk::Light => write!(f, "Light"),
        }
    }
}

pub struct DiskIter {
    index: usize,
}

impl DiskIter {
    pub fn new() -> Self {
        Self { index: 0 }
    }
}

impl Iterator for DiskIter {
    type Item = Disk;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index {
            0 => Some(Disk::Dark),
            1 => Some(Disk::Light),
            _ => None,
        }
    }
}
