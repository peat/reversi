mod board;
mod direction;
mod disk;
mod position;
mod transcript;

use crate::board::Board;
use crate::disk::Disk;
use crate::position::State;
use crate::transcript::Transcript;

fn main() {
    for _ in 0..1 {
        play_game();
    }
}

fn play_game() {
    let mut b = Board::default();
    b.pp();

    loop {
        let mut positions = Vec::new();
        for p in b.available_moves.keys() {
            positions.push(p.clone());
        }

        let mut options = Vec::new();
        for p in b.available_moves.keys() {
            options.push(Transcript::from(*p).format());
        }

        println!("");
        println!("Total Moves: {}", b.transcript.len());
        println!("Turn: {}", b.turn);
        println!("Available: {}", options.join(", "));
        println!("");

        b = match positions.first() {
            Some(p) => {
                // we have an available position; play it.
                b.play(p)
            }
            None => {
                // no available positions to play.
                match b.passed {
                    // if the previous player passed, the game is over.
                    true => break,
                    // if the previous player played, then current player passes.
                    false => b.pass(),
                }
            }
        };

        b.pp();
    }

    println!("Transcript: {}", Transcript::format_vec(&b.transcript));
    println!("Dark score: {}", b.score(&State::Occupied(Disk::Dark)));
    println!("Light score: {}", b.score(&State::Occupied(Disk::Light)));
    println!("");
}
