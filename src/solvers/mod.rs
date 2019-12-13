pub mod incremental;
pub mod random;
pub mod parallel;

use crate::game::{Game, ValidMove};

#[derive(Clone, Debug)]
struct Node {
    game: Game,
    valid_moves: Vec<ValidMove>,
}

impl Node {
    fn new(g: &Game) -> Node {
        let game = g.clone();
        let mut valid_moves = g.valid_moves();
        valid_moves.reverse(); // these are popped, so first used should be last in vec
        Node { game, valid_moves }
    }
}