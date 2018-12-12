use crate::game::{Game, ValidMoveIterator};

pub struct ResolveFirst {
    game: Game,
    move_iter: ValidMoveIterator,
}

impl ResolveFirst {
    fn update(&mut self, new_game: &Game) {
        self.game = new_game.clone();
        self.move_iter = new_game.valid_moves();
    }
}

impl From<Game> for ResolveFirst {
    fn from(game: Game) -> Self {
        let move_iter = game.valid_moves();
        Self { game, move_iter }
    }
}

impl Iterator for ResolveFirst {
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
