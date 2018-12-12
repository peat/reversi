mod board;
mod direction;
mod disk;
mod game;
mod position;
mod solvers;
mod transcript;

use crate::game::Game;
use crate::solvers::DepthFirst;
use crate::transcript::{Transcript, MANUBU_MARUO};

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

    let loops = 10_000;

    println!("Depth first solver ...\n");

    let game = Game::new();
    let mut dfs = DepthFirst::new(&game);
    let mut transcripts = Vec::new();

    let mut timer = Instant::now();

    for _ in 0..loops {
        let g = dfs.next().unwrap();
        transcripts.push(Transcript::stringify(&g.transcript))
    }

    println!(
        "  Generated {} complete games in {:?}",
        loops,
        timer.elapsed()
    );

    // Simple benchmark for playing the same game n times from a transcript
    let mut loop_results = Vec::new();

    timer = Instant::now();
    for t in transcripts {
        let transcript_vec = Transcript::from_string(&t);
        loop_results.push(Game::from_transcript(&transcript_vec))
    }

    println!("  Replayed from transcripts in {:?}", timer.elapsed());

    println!("\n------------\n");

    println!("TBD: Breadth first solver\n");
}
