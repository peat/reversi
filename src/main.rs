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

    println!("\n------------\n");

    // generate a String transcript of the original game
    let transcript = Transcript::stringify(&original_game.transcript);

    // replay the transcript on a fresh board, so we can check to see if it matches.
    println!("Generating board from transcript: {}\n", transcript);
    let copy_game = play_transcript(&transcript);
    copy_game.pp();

    if copy_game == original_game {
        println!("\n... They match!");
    } else {
        println!("\nNOT MATCHING!");
    }

    println!("\n------------\n");

    let loops = 1_000;
    println!("Benchmarking with {} replays ...", loops);
    timer = Instant::now();
    for _ in 0..loops {
        play_transcript(&transcript);
    }
    println!("Finished in {:?}\n", timer.elapsed());
}

fn play_transcript(transcript: &str) -> Board {
    let vec_t = Transcript::from_string(&transcript);
    Board::from_transcript(&vec_t)
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
