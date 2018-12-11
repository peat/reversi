use crate::disk::Disk;
use crate::position::Position;
use crate::transcript::Transcript;
use crate::grid::{Grid, State, MAX_X, MAX_Y};
use crate::analyzer::ValidMove;

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub transcript: Vec<Transcript>, // the history of plays on this board
    pub passed: bool,                // whether the last player passed
    pub light_count: usize,
    pub dark_count: usize,
    pub empty_count: usize,
    valid_moves: Vec<ValidMove>,    // the moves available to the current player
    pub grid: Grid,
}

impl Default for Board {
    // creates a new board, with starting positions filled and the first move
    // given to the Dark player.
    fn default() -> Self {
        let mut b = Board {
            transcript: Vec::new(),
            passed: false,
            valid_moves: Vec::new(),
            light_count: 2,
            dark_count: 2,
            empty_count: 60,
            grid: Grid::new(),
        };

        // set up the opening positions
        b = Board::set(b, &Position::new(3, 3), Some(Disk::Light));
        b = Board::set(b, &Position::new(4, 3), Some(Disk::Dark));
        b = Board::set(b, &Position::new(3, 4), Some(Disk::Dark));
        b = Board::set(b, &Position::new(4, 4), Some(Disk::Light));

        // populate the opening available moves
        // println!("Collecting initial moves ...");
        b.valid_moves = Board::moves_for(&b);
        // println!("... DONE!!");

        b // clean board in starting position!
    }
}

impl Board {

    // GAME PLAY METHODS ------------------------------------------------------

    pub fn play(board: &Board, position: &Position) -> Self {
        let mut new_board = board.clone();

        if Board::is_complete(board) {
            return new_board;
        }

        let affected = match Board::flips_for(board, position) {
            None => return new_board,
            Some(affected) => affected,
        };

        // flip 'em
        for a in affected.clone() {
            new_board = Board::flip(new_board, &a);
        }

        // mark the position as owned by the current player
        let occupied_by = Some(new_board.grid.turn);
        new_board = Board::set(new_board, &position, occupied_by);

        // record the move
        new_board.transcript.push(Transcript::from(*position));

        // on to the next turn!
        new_board.passed = false;
        Board::next_turn(new_board)
    }

    pub fn pass(board: &Board) -> Self {
        let mut new_board = board.clone();

        if Board::is_complete(&new_board) {
            return new_board;
        }

        new_board.passed = true;
        new_board.transcript.push(Transcript::Pass);
        Board::next_turn(new_board)
    }

    pub fn valid_moves(board: &Board) -> Option<Vec<Position>> {
        if board.valid_moves.is_empty() {
            return None;
        }

        Some(board.valid_moves.iter().map(|v| v.position).collect())
    }

    pub fn is_complete(board: &Board) -> bool {
        // there are no more Dark or Light disks on the board
        ((board.light_count == 0) 
            | (board.dark_count == 0))
        // or no available moves, and the last person passed -- complete!
        | (board.valid_moves.is_empty() && board.passed)
    }

    pub fn winner(board: &Board) -> Option<Disk> {
        if board.dark_count > board.light_count {
            return Some(Disk::Dark);
        }

        if board.light_count > board.dark_count {
            return Some(Disk::Light);
        }

        // it's a tie!
        None
    }

    pub fn transcript(board: &Board) -> String {
        Transcript::stringify(&board.transcript)
    }

    pub fn from_transcript(transcript: &[Transcript]) -> Self {
        let mut b = Board::default();

        let plays: Vec<Option<Position>> = transcript.iter().map(|t| t.to_position()).collect();

        for op in plays {
            b = match op {
                Some(p) => Board::play(&b, &p),
                None => Board::pass(&b),
            }
        }

        b
    }

    // INSPECTION METHODS -----------------------------------------------------

    pub fn pp(&self) {
        println!("  a b c d e f g h");
        for y in 0..8 {
            print!("{}", y + 1);
            for x in 0..8 {
                match Board::get(self, &Position { x, y }) {
                    None => print!(" •"),
                    Some(Disk::Dark) => print!(" D"),
                    Some(Disk::Light) => print!(" L"),
                }
            }
            println!();
        }
    }

    // PRIVATE METHODS --------------------------------------------------------

    fn get(board: &Board, position: &Position) -> Option<Disk> {
        match board.grid.get( position.x, position.y ) {
            State::Empty => None,
            State::Dark => Some(Disk::Dark),
            State::Light => Some(Disk::Light),
        }
    }

    fn set(mut board: Board, position: &Position, state: Option<Disk>) -> Self {
        let s = match state {
            None => State::Empty,
            Some(Disk::Dark) => State::Dark,
            Some(Disk::Light) => State::Light,
        };

        board.grid.set( position.x, position.y, s);
        board
    }

    fn count(board: &Board) -> (usize, usize, usize) {
        let mut dark_count = 0;
        let mut light_count = 0;
        let mut empty_count = 0;

        for y in 0..=MAX_Y {
            for x in 0..=MAX_X {
                match board.grid.get(x,y) {
                    State::Empty => empty_count += 1,
                    State::Dark => dark_count += 1,
                    State::Light => light_count += 1,
                }
            }
        }

        (dark_count, light_count, empty_count)
    }

    fn flips_for(board: &Board, position: &Position) -> Option<Vec<Position>> {
        for m in board.valid_moves.clone() {
            if m.position == *position {
                return Some(m.affected);
            }
        }
        None
    }

    fn flip(mut board: Board, position: &Position) -> Self {
        board.grid.flip(position.x, position.y);
        board
    }

    fn next_turn(mut board: Board) -> Board {
        board.grid.turn = board.grid.turn.opposite();
        board.valid_moves = Board::moves_for(&board);
        match Board::count(&board) {
            (d, l, e) => {
                board.dark_count = d;
                board.light_count = l;
                board.empty_count = e;
            }
        };
        board
    }

    fn moves_for(board: &Board) -> Vec<ValidMove> {
        let mut valid_moves = Vec::new();
        let mut vm_iterator = board.grid.moves();
        loop {
            match vm_iterator.next() {
                Some(vm) => valid_moves.push(vm),
                None => break,
            }
        }
        // println!("EH? {:?}", valid_moves);
        valid_moves
    }
}
