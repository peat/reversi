use crate::game::{Game, ValidMove};

struct Node {
    game: Game,
    valid_moves: Vec<ValidMove>,
}

impl Node {
    pub fn new(game: &Game) -> Self {
        let mut moves = game.valid_moves();
        moves.reverse();

        Node {
            game: game.clone(),
            valid_moves: moves, // so we can pop them off the end in order
        }
    }
}

pub struct DepthFirst {
    index: Vec<Node>,
}

impl DepthFirst {
    pub fn new(game: &Game) -> Self {
        let root = Node::new(game);
        DepthFirst { index: vec![root] }
    }

    fn trim(&mut self) {
        loop {
            match self.index.last() {
                None => return, // index is empty, yo
                Some(node) => {
                    if node.valid_moves.is_empty() {
                        // println!("      ... trimming empty node!");
                        let _ = self.index.pop();
                    } else {
                        return;
                    }
                }
            }
        }
    }
}

impl Iterator for DepthFirst {
    type Item = Game;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut node = match self.index.pop() {
                None => return None, // we have exhausted the set!
                Some(s) => s,
            };

            // let valid_positions: Vec<_> = node
            //     .valid_moves
            //     .iter()
            //     .map(|m| m.position.clone())
            //     .map(|p| format!("{}", Transcript::from(p)))
            //     .collect();
            // println!(
            //     "\nTrying node at position {}, with {} valid moves: {:?}",
            //     self.index.len(),
            //     node.valid_moves.len(),
            //     valid_positions
            // );
            // node.game.pp();

            match node.valid_moves.pop() {
                None => {
                    // println!("  ... no moves left");
                    // no moves remaining; check to see if the game is complete.
                    if node.game.is_complete() {
                        // revert up the stack until we find a node with available moves.
                        self.trim();
                        // println!("    ... complete! emitting!");
                        return Some(node.game);
                    } else {
                        // println!("    ... incomplete, passing.");
                        let new_game = node.game.pass();
                        // println!("    ... inserting new node at {}", self.index.len());
                        self.index.push(Node::new(&new_game))
                    }
                }
                Some(valid_move) => {
                    // play the move, stick the new game on the stack
                    // println!("  ... we have a move, playing.");
                    let new_game = node.game.play(valid_move);
                    // println!("    ... restoring updated node at {}", self.index.len());
                    self.index.push(node);
                    // println!("    ... inserting new node at {}", self.index.len());
                    self.index.push(Node::new(&new_game));
                }
            }
        }
    }
}

#[derive(Clone)]
struct NextSolver {
    game: Game,
}

impl NextSolver {
    pub fn new(game: Game) -> Self {
        game.into()
    }

    fn update(&mut self, new_game: Game) -> Option<Game> {
        self.game = new_game.clone();
        Some(new_game)
    }

    fn is_complete(&self) -> bool {
        self.game.is_complete()
    }
}

impl From<Game> for NextSolver {
    fn from(game: Game) -> Self {
        NextSolver { game }
    }
}

impl Iterator for NextSolver {
    type Item = Game;
    fn next(&mut self) -> Option<Self::Item> {
        match self.game.move_iter().next() {
            None => {
                if self.game.is_complete() {
                    None
                } else {
                    self.update(self.game.pass())
                }
            }
            Some(valid_move) => self.update(self.game.play(valid_move)),
        }
    }
}
