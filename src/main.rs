mod board;
mod direction;
mod disk;
mod game;
mod position;
mod solvers;
mod transcript;

use crate::game::Game;
use crate::solvers::ResolveFirst;
use crate::transcript::{Transcript, DEPTH_FIRST, MANUBU_MARUO};

use std::time::Instant;

fn main() {
    println!("\nDemos!");

    println!("\n------------\n");

    // Manubo Maruo's famous nine move game; shortest Othello victory possible
    println!("Replaying Manubu Maruo's 9 move win ...\n");

    let mm_transcript_vec = Transcript::from_string(MANUBU_MARUO);
    let mm_game = Game::from_transcript(&mm_transcript_vec);

    mm_game.pp();

    println!("\n------------\n");

    // Simple benchmark for playing the same game n times from a transcript
    let loops = 1_000;
    let mut loop_results = Vec::new();
    println!("Benchmarking with {} replays ...", loops);
    let mut timer = Instant::now();
    for _ in 0..loops {
        let transcript_vec = Transcript::from_string(DEPTH_FIRST);
        loop_results.push(Game::from_transcript(&transcript_vec))
    }
    println!("Finished {} in {:?}", loop_results.len(), timer.elapsed());

    println!("\n------------\n");

    println!("Simplest resolver, plays the first possible moves to completion ...\n");

    // play through an entire game, with random moves
    timer = Instant::now();

    let game = Game::new();

    let rfi: ResolveFirst = game.into();

    let finished_rfi = rfi.last().unwrap();

    finished_rfi.pp();

    println!("Generated in {:?}", timer.elapsed());

    println!("\n------------\n");

    println!("TBD: Breadth first solver\n");
}
