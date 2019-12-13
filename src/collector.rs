use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Instant;

use num_format::{Locale, ToFormattedString};

use crate::transcript::Transcript;

pub trait Collector {
    fn sender(&self) -> Sender<Vec<Transcript>>;
    fn start(&mut self);
}

pub struct Printer {
    _sender: Sender<Vec<Transcript>>,
    receiver: Receiver<Vec<Transcript>>,
}

impl Printer {
    pub fn new() -> Self {
        let (_sender, receiver): (Sender<Vec<Transcript>>, Receiver<Vec<Transcript>>) = mpsc::channel();
        Printer { _sender, receiver } 
    }
}

impl Collector for Printer {
    fn sender(&self) -> Sender<Vec<Transcript>> {
        self._sender.clone()
    }

    fn start(&mut self) {
        loop {
            match self.receiver.recv() {
                Ok(t) => println!("{}", Transcript::stringify(&t)),
                Err(e) => panic!("{}", e),
            }
        }
    }
}

const PRINT_COUNT: usize = 1_000_000;

pub struct Counter {
    count: usize,
    _sender: Sender<Vec<Transcript>>,
    receiver: Receiver<Vec<Transcript>>,
}

impl Counter {
    pub fn new() -> Self {
        let (_sender, receiver): (Sender<Vec<Transcript>>, Receiver<Vec<Transcript>>) = mpsc::channel();
        Counter { count: 0, _sender, receiver } 
    }
}

impl Collector for Counter {

    fn sender(&self) -> Sender<Vec<Transcript>> {
        self._sender.clone()
    }

    fn start(&mut self) {
        let timer = Instant::now();
        loop {
            match self.receiver.recv() {
                Ok(t) => {
                    self.count += 1;
                    if self.count % PRINT_COUNT == 0 {
                        let elapsed = timer.elapsed();
                        let total_games = self.count.to_formatted_string(&Locale::en);
                        let per_game = elapsed / (self.count as u32);
                        let per_second = (((self.count as f64) / elapsed.as_secs_f64()) as usize).to_formatted_string(&Locale::en);

                        println!("{} games in {:?} ({:?} per game, {} per second)", 
                            total_games, 
                            elapsed, 
                            per_game, 
                            per_second,
                        );
                        println!(" => {}", Transcript::stringify(&t));
                    }
                },
                Err(e) => panic!("{}", e),
            }
        }
    }
}