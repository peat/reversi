// width traversal of the game tree to queue up lots of games
// first branch of the tree is equivalent to the remainder
// - use the first starting position (D3)
// - play through subsequent games to a given depth
// - put those games in a queue, run 'em incremental until done

use crate::game::Game;
use crate::transcript::Transcript;
use crate::solvers::Node;

const SEED_MOVE_COUNT: usize = 6; // 2050 games

#[derive(Debug)]
pub struct Parallel {
    pub queue: Vec<Game>,
}

impl Parallel {
    pub fn new() -> Self {
        let starting_game = Game::from_transcript(Transcript::from_string("D3"));
        let root = Node::new(&starting_game);
        let mut queue = vec![root];
        let mut temp_queue = vec![];

        // iterate over the queue until we've reached the appropriate depth for our seed games
        for _ in 1..SEED_MOVE_COUNT {
            for node in queue {
                for mv in node.valid_moves {
                    temp_queue.push(Node::new(&node.game.play(mv)))
                }
            }
            queue = temp_queue;
            temp_queue = vec![];
        }

        Parallel { queue: queue.iter().map(|node| node.game.clone()).collect() }
    }
}

