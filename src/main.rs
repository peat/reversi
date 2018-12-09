mod board;
mod direction;
mod internal_position;
mod position_state;
mod transcript_position;

use crate::board::Board;
use crate::position_state::PositionState;
use crate::transcript_position::TranscriptPosition;

fn main() {
    for _ in 1..100 {
        play_game();
    }
}

fn play_game() {
    let mut b = Board::default();

    loop {
        let available_moves = b.moves_for(b.turn);
        let mut positions = Vec::new();
        for p in available_moves.keys() {
            positions.push(p.clone());
        }

        match positions.first() {
            Some(p) => {
                // we have an available position; play it.
                let i = *p;
                let t: TranscriptPosition = i.into();
                b.play(t);
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
        }
    }

    b.pp();
    println!("Transcript: {}", Board::fmt_points(b.moves.clone()));
    println!("Dark score: {}", b.in_state(PositionState::Dark).len());
    println!("Light score: {}", b.in_state(PositionState::Light).len());
    println!("");
}
