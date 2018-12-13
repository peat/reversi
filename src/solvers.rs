use crate::game::{Game, ValidMove};
use rand::{thread_rng, Rng};

pub struct NodeBuilder {}

impl NodeBuilder {
    pub fn left(g: &Game) -> Node {
        let game = g.clone();
        let mut valid_moves = g.valid_moves();
        valid_moves.reverse(); // these are popped from the vec, so first used should be last in vec
        Node { game, valid_moves }
    }

    pub fn right(g: &Game) -> Node {
        let game = g.clone();
        let valid_moves = g.valid_moves();
        Node { game, valid_moves }
    }

    pub fn random(g: &Game) -> Node {
        let game = g.clone();
        let mut valid_moves = g.valid_moves();
        thread_rng().shuffle(&mut valid_moves);
        Node { game, valid_moves }
    }
}

pub struct Node {
    game: Game,
    valid_moves: Vec<ValidMove>,
}

pub struct DepthFirstIterator {
    node_builder: fn(&Game) -> Node,
    index: Vec<Node>,
}

impl DepthFirstIterator {
    pub fn new(node_builder: fn(&Game) -> Node, game: &Game) -> Self {
        let root = node_builder(game);
        DepthFirstIterator {
            node_builder: node_builder,
            index: vec![root],
        }
    }

    fn new_node(&self, game: &Game) -> Node {
        (self.node_builder)(game)
    }

    fn trim(&mut self) {
        loop {
            match self.index.last() {
                None => return,
                Some(node) => {
                    if node.valid_moves.is_empty() {
                        let _ = self.index.pop();
                    } else {
                        return;
                    }
                }
            }
        }
    }
}

impl Iterator for DepthFirstIterator {
    type Item = Game;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut current_node = match self.index.pop() {
                None => return None, // we have exhausted the set!
                Some(s) => s,
            };

            match current_node.valid_moves.pop() {
                None => {
                    // no moves remaining; check to see if the game is complete.
                    if current_node.game.is_complete() {
                        // revert up the stack until we find a node with available moves.
                        self.trim();
                        return Some(current_node.game);
                    } else {
                        let new_game = current_node.game.pass();
                        self.index.push(self.new_node(&new_game))
                    }
                }
                Some(valid_move) => {
                    // play the move, return the current node to the stack,
                    // and stick the new game on the stack
                    let new_game = current_node.game.play(valid_move);
                    self.index.push(current_node);
                    self.index.push(self.new_node(&new_game));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcript::Transcript;

    #[test]
    fn verify_left_depth_first_iterator() {
        let expected_transcript = "D3C3B3B2B1A1C4C1C2D2D1E1A2A3F5E2F1G1PPF2PPE3PPB5B4A5A4C5A6F4F3G3G2H2H1H3H4G4C6G5H5B6C7D6E6F6G6H6H7A7PPB7A8D7E7F7G7G8B8C8D8E8F8H8";
        let mut depth_first_iterator = DepthFirstIterator::new(NodeBuilder::left, &Game::new());
        let first_result = depth_first_iterator.next().unwrap();

        assert_eq!(
            Transcript::from_string(expected_transcript),
            first_result.transcript
        );
    }

}
