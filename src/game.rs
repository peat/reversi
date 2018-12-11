use crate::board::Board;
use crate::position::Position;

pub enum Play {
    Place(Position),
    Pass,
}

pub struct Game {
    board: Board,
    available_moves: Vec<Play>,
}

impl Game {
    pub fn new(board: &Board) -> Self {
        // handle boards that are already complete.
        if Board::is_complete(board) {
            return Game {
                board: board.clone(),
                available_moves: Vec::new(),
            };
        }

        let available_moves = match board.available_moves.len() {
            // if there are no available moves on the board, our only option is to pass.
            0 => vec![Play::Pass],

            // collect all of the available moves ...
            _ => board
                .available_moves
                .keys()
                .map(|p| Play::Place(*p))
                .collect(),
        };

        Game {
            board: board.clone(),
            available_moves,
        }
    }

    pub fn random(board: &Board) -> Board {
        let mut b = board.clone();
        loop {
            b = match b.available_moves.keys().next() {
                Some(p) => {
                    // we have an available position; play it.
                    Board::play(&b, p)
                }
                None => {
                    // no available positions to play.
                    if Board::is_complete(&b) {
                        // and the game is over ...
                        break b;
                    } else {
                        // no moves, but the game isn't over, so the current player passes.
                        Board::pass(&b)
                    }
                }
            };
        }
    }

    pub fn recurse(board: &Board, depth: usize) -> Vec<Board> {
        let mut boards = vec![board.clone()];

        for _ in 0..depth {
            let mut results = Vec::new();
            for b in &boards {
                for result in Game::new(&b) {
                    results.push(result);
                }
            }
            if !results.is_empty() {
                boards = results;
            } else {
                break;
            }
        }
        boards
    }
}

impl Iterator for Game {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        match self.available_moves.pop() {
            None => None,
            Some(p) => match p {
                Play::Pass => Some(Board::pass(&self.board)),
                Play::Place(o) => Some(Board::play(&self.board, &o)),
            },
        }
    }
}
