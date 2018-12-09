#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PositionState {
    Empty,
    Light,
    Dark,
}

impl PositionState {
    pub fn opposite(b: PositionState) -> PositionState {
        match b {
            PositionState::Empty => PositionState::Empty,
            PositionState::Dark => PositionState::Light,
            PositionState::Light => PositionState::Dark,
        }
    }
}

impl Default for PositionState {
    fn default() -> Self {
        PositionState::Empty
    }
}
