use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
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
