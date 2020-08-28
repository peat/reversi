use crate::game::Game;
use rand::prelude::*;
use sha2::{Digest, Sha256};

pub struct Seed {
    pub string: String,
    value: [u8; 32], // 256 bits, for StdRng::from_seed
}

impl Seed {
    const SEED_LENGTH: usize = 32;

    // the default seed is a SHA256 hash of the word "reversi"
    pub fn new() -> Self {
        Seed::from_string("reversi".to_string())
    }

    pub fn from_string(string: String) -> Self {
        let mut hasher = Sha256::default();
        hasher.update(string.clone());
        let result = hasher.finalize();
        let mut value: [u8; 32] = [0; 32];
        for idx in 0..Self::SEED_LENGTH {
            value[idx] = result[idx];
        }

        Self { string, value }
    }
}

pub struct Random {
    game: Game,
    rng: StdRng,
}

impl Random {
    pub fn new(game: Game, seed: Seed) -> Self {
        Self {
            game,
            rng: StdRng::from_seed(seed.value),
        }
    }

    fn solve(&mut self) -> Game {
        let mut g = self.game.clone();
        loop {
            if g.is_complete() {
                return g;
            }

            let valid_moves = g.valid_moves();

            g = match valid_moves.choose(&mut self.rng) {
                None => g.pass(),
                Some(vm) => g.play(vm.clone()),
            }
        }
    }
}

impl Iterator for Random {
    type Item = Game;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.solve())
    }
}
