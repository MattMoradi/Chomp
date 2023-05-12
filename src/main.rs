//! Command-line Chomp player  
//! Matt Moradi and Bart Massey 2023
//!
//! This player repeatedly
//! * Displays the board
//! * Prompts the human for a move until a legal move is obtained
//! * Makes the human move on the board
//! * Displays the board
//! * Gets a winning computer move from the AI
//!   * If the AI has no winning move, chooses a computer move
//!     by going to the last available row and eating the last
//!     available square in that row
//! * Makes the computer move on the board
//! * Displays the computer move
//! This continues until the game is over,
//! at which point either "you lose" or "you win"
//! is printed depending on the outcome.

use chomp_ai::*;
use prompted::input;
use std::env;

/// Display the current board. This should produce output in this format:
///
///    #####
///    #####
///    ####.
///    #....
///
fn show_posn(posn: &Chomp) 
{
    println!();
    let mut columns = 0;

    while columns < posn.ncols
    {
        let mut rows = 0;
        while rows < posn.nrows
        {
            if posn.board[rows][columns] {print!("#");}
            else {print!(".")};
            rows += 1;
        }
        println!();
        columns += 1;
    }
    println!();
}

/// Get a move from the human player. The human should
/// supply the move as a row and column (starting from 0)
/// separated by a space, like this.
///
///    2 3
///
/// If the human makes a "bad" move (badly formatted or
/// illegal), this function returns `None`. Otherwise it
/// returns `Some` row and column coordinates of the human
/// move.
fn user_move(posn: &Chomp) -> Option<(usize, usize)> 
{
    let input = input!("Your Move <x> <y>: ");
    let points = input.trim().split(' ').flat_map(str::parse::<usize>).collect::<Vec<_>>();
    if points.len() >= 2 && posn.board[points[1]][points[0]] {Some((points[0], points[1]))}
    else {None}
}

/// Usage error manager
fn error() -> !
{
    eprintln!("chomp usage: chomp <x> <y>");
    std::process::exit(1);
}

/// Parse command line input
fn parsenum(s: &str) -> usize
{
    s.parse().unwrap_or_else(|_| error())
}

/// Play a game, as described above.
/// 
/// The program should take two command-line arguments
/// representing the board size: a number of rows and a
/// number of columns for the board. The program should fail
/// (somehow) if the requested board size is too large or
/// negative or not numbers etc.
///
/// Thus, a typical run of the program on a 3×4 board might look like
/// ```text
/// cargo run 3 4
/// ```
fn main() 
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {error()};

    let x = parsenum(&args[1]);
    let y = parsenum(&args[2]);
    
    let mut c = Chomp::new(x, y);

    loop
    {
        show_posn(&c);
        loop
        {
            let m = user_move(&c);
            if m.is_none()
            {
                println!("BAD SELECTION!");
                continue;
            }

            if let Some((a,b)) = m
            {
                c.make_move(a, b);
                break;
            }
        }

        show_posn(&c);
        
        // poison square eaten on user move
        if !c.board[0][0]
        {
            println!("YOU LOSE!");
            break;
        }

        let m = c.winning_move();

        if m.is_none()
        {
            let mut row = 0;
            let mut col = 0;

            while row < c.nrows && c.board[row][0]
            {
                row += 1;
            }
            while col < c.ncols && c.board[row][col]
            {
                col += 1;
            }
            c.make_move(row, col);
        }
        else if let Some((a,b)) = m 
        {
            c.make_move(a, b);
        }
        println!("AI Moved: ");

        // poison square eaten on AI move
        if !c.board[0][0]
        {
            println!("YOU WIN!");
            break;
        }
    }
}
