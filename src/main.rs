mod board;
mod contest;
mod direction;
mod disk;
mod game;
mod position;
mod solvers;
mod transcript;

extern crate rand;
extern crate sha2;

use crate::contest::{Contest, Player};
use crate::game::Game;
use crate::solvers::incremental::Incremental;
use crate::solvers::random::{Random, Seed};
use crate::transcript::{Transcript, MANUBU_MARUO};

use std::env;
use std::time::Instant;

fn main() {
    match env::args().last() {
        None => help(),
        Some(raw_mode) => match raw_mode.to_ascii_lowercase().trim() {
            "demos" => demos(),
            "incremental" => incremental(),
            "random" => random(),
            _ => help(),
        },
    }
}

fn help() {
    println!("usage: reversi [option]");
    println!();
    println!("Available options:");
    println!();
    println!("  demos            Spits out a series of demos and benchmarking info.");
    println!("  incremental      Generates non-repeating transcripts.");
    println!("  random           Generates random transcripts.");
    println!("  help             This screen.");
    println!();
}

fn incremental() {
    let game = Game::new();
    let mut s = Incremental::new(&game);
    loop {
        match s.next() {
            None => return,
            Some(result) => println!("{}", Transcript::stringify(&result.transcript)),
        }
    }
}

fn random() {
    let game = Game::new();
    let seed = Seed::new();
    eprintln!(
        "Generating random games from seed \"{}\"",
        seed.string.clone()
    );
    let mut s = Random::new(game, seed);
    loop {
        match s.next() {
            None => return,
            Some(result) => println!("{}", Transcript::stringify(&result.transcript)),
        }
    }
}

fn demos() {
    println!("\nDemos!");

    println!("\n------------\n");

    // Manubo Maruo's famous nine move game; shortest Othello victory possible
    println!("Replaying Manubu Maruo's 9 move win ...\n");

    let mm_transcript_vec = Transcript::from_string(MANUBU_MARUO);
    let mm_game = Game::from_transcript(&mm_transcript_vec);

    mm_game.pp();

    println!("\n------------\n");

    println!("Generating the first incremental completed game ...");

    let mut timer = Instant::now();
    let mut game = Game::new();
    let mut iterator = Incremental::new(&game);
    game = iterator.next().unwrap();
    let mut elapsed = timer.elapsed();

    game.pp();
    println!("Elapsed: {:?}", elapsed);

    println!("\nAnd second incremental completed game ...");

    timer = Instant::now();
    game = iterator.next().unwrap();
    elapsed = timer.elapsed();

    game.pp();
    println!("Elapsed: {:?}", elapsed);

    println!("\nAnd rotate it (identical solution) ...");
    timer = Instant::now();
    let rotated = game.to_rotated();
    elapsed = timer.elapsed();

    rotated.pp();
    println!("Elapsed: {:?}", elapsed);

    println!("\n------------\n");

    let loops = 1_000;

    println!(
        "Benchmark the depth first solver and transcripts ({} games)...\n",
        loops
    );

    game = Game::new();
    let mut dfs = Incremental::new(&game);
    let mut transcripts = Vec::new();

    timer = Instant::now();

    for _ in 0..loops {
        let g = dfs.next().unwrap();
        transcripts.push(Transcript::stringify(&g.transcript))
    }

    println!("  Generated games in {:?}", timer.elapsed());

    // Simple benchmark for playing the same game n times from a transcript
    let mut loop_results = Vec::new();

    timer = Instant::now();
    for t in transcripts {
        let transcript_vec = Transcript::from_string(&t);
        loop_results.push(Game::from_transcript(&transcript_vec))
    }

    println!("  Replayed from transcripts in {:?}", timer.elapsed());

    println!("\n------------\n");

    println!("Playing through a basic contest (first moves) ...\n ");

    println!("Result:");
    let mut contest = Contest::new(Game::new(), Player::new(), Player::new());

    timer = Instant::now();
    contest.play();
    elapsed = timer.elapsed();

    contest.game.pp();
    println!("Elapsed: {:?}", elapsed);

    println!();
}
