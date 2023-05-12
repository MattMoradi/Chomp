//! Chomp AI
//! Matt Moradi and Bart Massey 2023

/// Maximum number of rows the AI can handle.
const MAX_ROWS: usize = 4;
/// Maximum number of columns the AI can handle.
const MAX_COLS: usize = 5;

/// A Chomp board.
#[derive(Debug, Clone)]
pub struct Chomp {
    /// The number of rows for this game.
    pub nrows: usize,

    /// The number of columns for this game.
    pub ncols: usize,

    /// The board. `true` is an un-eaten square, `false is
    /// an eaten square.
    pub board: [[bool; MAX_COLS]; MAX_ROWS],
}

impl Chomp {
    /// Make a new Chomp board with the given size for this game.
    ///
    /// # Panics
    /// Panics if the requested board size is larger than the AI
    /// can handle, or has zero rows or columns.
    pub fn new(nrows: usize, ncols: usize) -> Self {
        assert!(nrows > 0, "not enough rows to play");
        assert!(ncols > 0, "not enough columns to play");
        assert!(
            nrows <= MAX_ROWS,
            "too many rows ({} > {}) for AI",
            nrows,
            MAX_ROWS
        );
        assert!(
            ncols <= MAX_COLS,
            "too many columns ({} > {}) for AI",
            ncols,
            MAX_COLS
        );

        Chomp {nrows: nrows, ncols: ncols, board: [[true; MAX_COLS]; MAX_ROWS]}
    }

    /// Make a move on the current board, "eating" all cells
    /// below `row` and to the right of `col` inclusive.
    pub fn make_move(&mut self, row: usize, col: usize) 
    {
        let mut rows = row;

        while rows < self.nrows
        {
            let mut columns = col;
            while columns < self.ncols
            {
                self.board[rows][columns] = false;
                columns += 1;
            }
            println!();
            rows += 1;
        }
    }

    /// Returns `Some` winning move for this position as `(row, col)`.
    /// Returns `None` if there is no winning move in this position.
    ///
    /// # Strategy
    ///
    /// ```text
    /// winning-move(posn):
    ///     for each remaining row r
    ///         for each remaining column c in r
    ///             if r = 0 and c = 0
    ///                 continue
    ///             p ← copy of posn
    ///             chomp r, c from p
    ///             m ← winning-move(p)
    ///             if no winning move is returned
    ///                 return the move r, c
    ///    return no winning move
    /// ```
    pub fn winning_move(&self) -> Option<(usize, usize)> {
        let mut r = 0;

        while r < self.nrows && self.board[r][0] == true
        {
            let mut c = 0;
            while c < self.ncols && self.board[r][c] == true
            {
                println!("{}", c);
                if r == 0 && c == 0
                {
                    c+=1;
                    continue;
                }

                let mut p = self.clone();
                p.make_move(r, c);
                let m = p.winning_move();
                println!("HERE");
                //if m == Some((r, c))
                if(m.is_none())
                {
                    return Some((r, c));
                }
                c+=1;
            }
            r+=1;
        }
        None
    }
}

#[test]
fn test_winning_move() {
    let mut c = Chomp::new(2, 2);
    assert!(c.winning_move().is_some());
    c.make_move(1, 1);
    assert!(c.winning_move().is_none());
}
