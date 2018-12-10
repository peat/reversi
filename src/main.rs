mod board;
mod direction;
mod disk;
mod position;
mod transcript;

use crate::board::Board;
use crate::disk::Disk;
use crate::transcript::Transcript;

use std::time::Instant;

fn main() {
    // play through an entire game, with random moves
    let mut timer = Instant::now();
    let original_game = generate_game();
    println!("Generated in {:?}", timer.elapsed());

    println!("\n------------");

    // generate a String transcript of the original game
    let transcript = Transcript::stringify(&original_game.transcript);

    // replay the transcript on a fresh board, so we can check to see if it matches.
    timer = Instant::now();
    let copy_game = play_transcript(&transcript);
    println!("Replayed in {:?}", timer.elapsed());

    if copy_game == original_game {
        println!("... They match!");
    } else {
        println!("NOT MATCHING!");
    }
}

fn play_transcript(transcript: &str) -> Board {
    println!();
    println!("Generating board from transcript: {}", transcript);
    println!();

    let vec_t = Transcript::from_string(&transcript);
    let b = Board::from_transcript(&vec_t);

    b.pp();
    println!();
    b
}

fn generate_game() -> Board {
    let mut b = Board::default();
    b.pp();

    loop {
        let mut positions = Vec::new();
        let mut options = Vec::new();

        for p in b.available_moves.keys() {
            positions.push(p.clone());
            options.push(Transcript::from(*p).format());
        }

        println!();
        println!("Total Moves: {}", b.transcript.len());
        println!("Turn: {}", b.turn);
        println!("Available: {}", options.join(", "));
        println!();

        b = match positions.first() {
            Some(p) => {
                // we have an available position; play it.
                Board::play(&b, p)
            }
            None => {
                // no available positions to play.
                if b.passed {
                    // if the previous player passed, the game is over.
                    break;
                } else {
                    // if the previous player played, then current player passes.
                    Board::pass(&b)
                }
            }
        };

        b.pp();
    }

    println!("Transcript: {}", Transcript::stringify(&b.transcript));
    println!("Dark score: {}", Board::score(&b, Disk::Dark));
    println!("Light score: {}", Board::score(&b, Disk::Light));
    println!();

    b
}
