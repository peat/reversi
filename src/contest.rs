use crate::disk::Disk;
use crate::game::Game;

pub struct Contest {
    pub game: Game,
    dark: Player,
    light: Player,
}

impl Contest {
    pub fn new(game: Game, mut dark: Player, mut light: Player) -> Self {
        dark.disk = Disk::Dark;
        light.disk = Disk::Light;
        Self { game, dark, light }
    }

    pub fn play(&mut self) {
        while !self.game.is_complete() {
            let player = match self.game.turn {
                Disk::Dark => &mut self.dark,
                Disk::Light => &mut self.light,
            };

            let playable_game = self.game.clone();
            let result = player.play(playable_game);
            self.game = result;
        }
    }
}

#[derive(Clone)]
pub struct Player {
    pub disk: Disk,
}

impl Player {
    pub fn new() -> Self {
        Self { disk: Disk::Dark }
    }

    pub fn play(&mut self, game: Game) -> Game {
        match game.move_iter().next() {
            Some(m) => game.play(m),
            None => game.pass(),
        }
    }
}
