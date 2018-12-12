use crate::game::{Game, ValidMoveIterator};

pub struct DepthFirst {
    history: Vec<Game>,
    game: Game,
    move_iter: ValidMoveIterator,
}

impl DepthFirst {
    fn update(&mut self, new_game: &Game) {
        self.history.push(self.game.clone());
        self.game = new_game.clone();
        self.move_iter = new_game.valid_moves();
    }
}

impl From<Game> for DepthFirst {
    fn from(game: Game) -> Self {
        let history = Vec::new();
        let move_iter = game.valid_moves();
        DepthFirst {
            history,
            game,
            move_iter,
        }
    }
}

impl Iterator for DepthFirst {
    type Item = Game;
    fn next(&mut self) -> Option<Self::Item> {
        match self.move_iter.next() {
            None => {
                if self.game.is_complete() {
                    None
                } else {
                    let new_game = self.game.pass();
                    self.update(&new_game);
                    Some(new_game)
                }
            }
            Some(valid_move) => {
                let new_game = self.game.play(valid_move);
                self.update(&new_game);
                Some(new_game)
            }
        }
    }
}
