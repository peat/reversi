mod board;
mod direction;
mod disk;
mod game;
mod position;
mod transcript;

use crate::board::Board;
use crate::disk::Disk;
use crate::game::Game;
use crate::transcript::Transcript;

use std::time::Instant;

fn main() {
    // play through an entire game, with random moves
    let mut timer = Instant::now();
    let board = generate_game();
    println!("Generated in {:?}", timer.elapsed());

    println!("\n------------\n");

    // generate a String transcript of the original game
    let transcript = Board::transcript(&board);

    // Simple benchmark for playing the same game n times from a transcript
    let loops = 1_000;
    println!("Benchmarking with {} replays ...", loops);
    timer = Instant::now();
    for _ in 0..loops {
        play_transcript(&transcript);
    }
    println!("Finished in {:?}", timer.elapsed());

    println!("\n------------\n");

    // Manubo Maruo's famous nine move game; shortest Othello victory possible
    println!("Replaying Manubu Maruo's 9 move win ...\n");

    let mm_transcript = "E6F4E3F6G5D6E7F5C5";
    let mm_transcript_vec = Transcript::from_string(mm_transcript);
    let mm_board = Board::from_transcript(&mm_transcript_vec);

    mm_board.pp();

    println!("\n-> {}", Transcript::stringify(&mm_board.transcript));

    // sanity check on completeness
    if !Board::is_complete(&mm_board) {
        println!("Oh no! This isn't complete??");
    }

    println!("\n------------\n");

    // start the breadth first solver; go two levels deep
    let mut rounds = 3;
    println!("Iterating through {} rounds ...", rounds);

    timer = Instant::now();
    let boards = Game::recurse(&Board::default(), rounds);

    println!("Generated {} games in {:?}", boards.len(), timer.elapsed());

    println!("\n------------\n");

    // Start with a nearly complete game -- figure out all of the different completions.
    let partial = "E6F4F3F6G4G3G2E3D3H1G6H2E2D2G1E7E8F1F5H3C2C6C4F7G5H7H5H4F2B4A4D7B5D8G7D1C8D6C3F8C1C7B8E1C5A6B3A2A5A8";
    rounds = 15;
    println!("Finding completions, starting with: {}", partial);

    let partial_tv = Transcript::from_string(partial);
    let partial_board = Board::from_transcript(&partial_tv);

    println!("There are {} open positions ... \n", board.empty_count);

    partial_board.pp();

    let finished = Game::recurse(&partial_board, rounds);

    let mut completed = 0;
    let mut dark_wins = 0;
    let mut light_wins = 0;
    let mut ties = 0;
    for b in &finished {
        if Board::is_complete(&b) {
            completed += 1;
            match Board::winner(&b) {
                None => ties += 1,
                Some(d) => match d {
                    Disk::Dark => dark_wins += 1,
                    Disk::Light => light_wins += 1,
                },
            }
        }
    }

    println!(
        "\nPlayed {}, completed {} games: Dark {}, Light {}, tied {}",
        finished.len(),
        completed,
        dark_wins,
        light_wins,
        ties
    );

    println!();
}

fn play_transcript(transcript: &str) -> Board {
    let vec_t = Transcript::from_string(transcript);
    Board::from_transcript(&vec_t)
}

fn generate_game() -> Board {
    let b = Game::random(&Board::default());

    b.pp();

    println!("Transcript: {}", Board::transcript(&b));
    println!("Dark score: {}", b.dark_count);
    println!("Light score: {}", b.light_count);
    println!();

    b
}
