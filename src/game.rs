use crate::board::Board;
use crate::position::Position;

pub struct Game {
    board: Board,
    available_moves: Vec<Option<Position>>,
}

pub struct BreadthRecursion {
    board: Board,
    available_moves: Vec<Option<Position>>,
}

impl BreadthRecursion {
    pub fn from(board: &Board) -> Self {
        let game = Game::new(board);

        BreadthRecursion {
            board: game.board,
            available_moves: game.available_moves,
        }
    }
}

impl Iterator for BreadthRecursion {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        match self.available_moves.pop() {
            None => None,
            Some(p) => match p {
                None => Some(Board::pass(&self.board)),
                Some(o) => Some(Board::play(&self.board, &o)),
            },
        }
    }
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

        let available_moves = match Board::valid_moves(board) {
            // if there are no available moves on the board, our only option is to pass.
            None => vec![None],

            // collect all of the available moves ...
            Some(ps) => ps.iter().map(|p| Some(*p)).collect(),
        };

        Game {
            board: board.clone(),
            available_moves,
        }
    }

    pub fn random(board: &Board) -> Board {
        let mut b = board.clone();
        loop {
            b = match Board::valid_moves(&b) {
                Some(ps) => {
                    // we have an available position; play it.
                    Board::play(&b, ps.first().unwrap())
                }
                None => {
                    // no available positions to play.
                    if Board::is_complete(&b) {
                        // and the game is over ...
                        return b;
                    } else {
                        // no moves, but the game isn't over, so the current player passes.
                        Board::pass(&b)
                    }
                }
            };
        }
    }

    pub fn breadth_recursion(board: &Board, depth: usize) -> Vec<Board> {
        let mut boards = vec![board.clone()];

        for _ in 0..depth {
            let mut results = Vec::new();
            for b in &boards {
                for result in BreadthRecursion::from(&b) {
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
