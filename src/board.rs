use crate::direction::Direction;
use crate::disk::Disk;
use crate::position::Position;
use crate::transcript::Transcript;
use crate::grid::{Grid, GridIterator, State, MAX_X, MAX_Y};

#[derive(Clone, Debug, Ord, PartialOrd, Hash, Eq, PartialEq)]
struct ValidMove {
    position: Position,
    affected: Vec<Position>
}

struct MoveIterator {
    grid_iterator: GridIterator
}

impl Iterator for MoveIterator {
    type Item = ValidMove;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.grid_iterator.next() {
                Some((position, State::Empty)) => {
                ...
                } 
                None => return None,
                _ => continue,
            }
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub transcript: Vec<Transcript>, // the history of plays on this board
    pub turn: Disk,                  // who is currently playing
    pub passed: bool,                // whether the last player passed
    pub light_count: usize,
    pub dark_count: usize,
    pub empty_count: usize,
    valid_moves: Vec<ValidMove>,    // the moves available to the current player
    grid: Grid,
}

impl Default for Board {
    // creates a new board, with starting positions filled and the first move
    // given to the Dark player.
    fn default() -> Self {
        let mut b = Board {
            transcript: Vec::new(),
            turn: Disk::Dark,
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
        b.valid_moves = Board::moves_for(&b, b.turn);

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
        let occupied_by = Some(new_board.turn);
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

    fn in_state(board: &Board, state: State) -> Vec<Position> {
        let mut output = Vec::new();
        for y in 0..=MAX_Y {
            for x in 0..=MAX_X {
                if board.grid.get(x,y) == state {
                    output.push(Position { x, y} );
                }
            }
        }
        output
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
        board.turn = board.turn.opposite();
        board.valid_moves = Board::moves_for(&board, board.turn);
        match Board::count(&board) {
            (d, l, e) => {
                board.dark_count = d;
                board.light_count = l;
                board.empty_count = e;
            }
        };
        board
    }

    fn attempt_direction(
        board: &Board,
        position: &Position,
        player_disk: Disk,
        direction: &Direction,
    ) -> Option<Vec<Position>> {
        let mut found_positions = Vec::new();

        // placeholder while we're traversing the board
        let mut current_position = *position;

        loop {
            // see if we can load the next position in the given direction
            match current_position.direction(direction) {
                // whoops, hit an edge; nothing in this direction!
                None => return None,

                // ok, we found a neighbor. let's check it out ...
                Some(neighbor) => {
                    match Board::get(board, &neighbor) {
                        // Aww dang, we hit an empty space; abort!
                        None => return None,
                        Some(d) => {
                            // if our neighbor is in the same state as the player,
                            // it MIGHT mean we've been collecting flippable positions!
                            if d == player_disk {
                                // if we've found flippable positions, return 'em, otherwise
                                // return nothing.
                                if found_positions.is_empty() {
                                    return None;
                                } else {
                                    return Some(found_positions);
                                }
                            } else {
                                // if our neighbor is in an opposing state, it it could be a flippable
                                // position! Collect it, and step into the neighbor position.
                                found_positions.push(neighbor);
                                current_position = neighbor;
                            }
                        }
                    }
                }
            }
        }
    }

    fn moves_for(board: &Board, disk: Disk) -> Vec<ValidMove> {
        // placeholder for our playable set
        let mut playable_set = Vec::new();

        // gather all of the empty, playable spaces
        let empty_positions = Board::in_state(board, State::Empty);

        // step through each empty position and determine if it is playable.
        for position in empty_positions {
            // we'll collect all of the positions that would be flipped by a play at the
            // current position here
            let mut affected = Vec::new();

            // cycle through our cardinal directions
            for direction in &Direction::ALL {
                // if we find a play, collect the potentially affected positions
                if let Some(a) = Board::attempt_direction(board, &position, disk, direction)
                {
                    affected.extend(a);
                }
            }

            // done testing all of the directions; if we have anything, stash it and move on ...
            if !affected.is_empty() {
                let valid_move = ValidMove { position, affected };
                playable_set.push(valid_move);
            }
        }
        playable_set
    }
}
