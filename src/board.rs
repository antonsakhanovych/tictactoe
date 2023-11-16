use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Square {
    EMPTY,
    X,
    O,
}

const WINNING_COMBINATIONS: [[(usize, usize); 3]; 8] = [
    // Horizontal combinations
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    // Vertical combinations
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    // Diagonal combinations
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

struct Move(u8, u8);

#[derive(Debug)]
pub struct Board {
    board: [[Square; 3]; 3],
}

impl Board {
    pub fn create_from(board: [[Square; 3]; 3]) -> Self {
        Self { board }
    }

    pub fn new() -> Self {
        Self {
            board: [
                [Square::EMPTY, Square::EMPTY, Square::EMPTY],
                [Square::EMPTY, Square::EMPTY, Square::EMPTY],
                [Square::EMPTY, Square::EMPTY, Square::EMPTY],
            ],
        }
    }
    pub fn terminal(&self) -> bool {
        if self.num_empty() == 0 || self.get_winner() != None {
            true
        } else {
            false
        }
    }

    pub fn num_empty(&self) -> usize {
        self.board
            .iter()
            .flatten()
            .filter(|&el| *el == Square::EMPTY)
            .count()
    }

    pub fn curr_player(&self) -> Square {
        let num_empty = self.num_empty();
        if num_empty % 2 == 0 {
            Square::O
        } else {
            Square::X
        }
    }
    pub fn get_winner(&self) -> Option<Square> {
        for comb in WINNING_COMBINATIONS {
            let row = comb
                .iter()
                .map(|&(row_ind, col_ind)| self.board[row_ind][col_ind])
                .collect::<Vec<Square>>();

            if row.iter().all(|&square| square == Square::X) {
                return Some(Square::X);
            } else if row.iter().all(|&square| square == Square::O) {
                return Some(Square::O);
            }
        }
        None
    }
}

#[test]
fn test_winner() {
    let b = Board::create_from([
        [Square::X, Square::EMPTY, Square::EMPTY],
        [Square::EMPTY, Square::EMPTY, Square::EMPTY],
        [Square::EMPTY, Square::EMPTY, Square::EMPTY],
    ]);

    let res = b.get_winner();
    println!("{b:?}");
    println!("{res:?}");
}
