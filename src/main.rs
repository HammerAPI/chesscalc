use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

/// Represents a `rows` by `cols` board.
struct Grid {
    board: Vec<usize>,
    cols: usize,
    _rows: usize,
    size: usize,
}

impl Grid {
    /// Constructs a new `Grid` from the `filename` input file.
    ///
    /// # Panics
    ///
    /// This function panics if the supplied file cannot be found.
    fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("Could not find file");
        let reader = BufReader::new(file);

        let mut row: Vec<usize>;
        let mut _rows = 0;
        let mut cols = 0;
        let mut board = Vec::new();
        for line in reader.lines() {
            row = line
                .unwrap_or(String::new())
                .split_whitespace()
                .filter(|c| c.parse::<usize>().is_ok())
                .map(|s| s.parse().unwrap())
                .collect();
            _rows += 1;
            cols = row.len();
            board.append(&mut row);
        }

        let size = board.len();

        Grid {
            board,
            cols,
            _rows,
            size,
        }
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: `cargo run <input file>`");
        exit(1);
    }

    let grid = Grid::new(&args[1]);

    display_board(&grid);

    for col in 0..grid.cols {
        println!("\nCOL #{} SUMS:", col);
        println!("\tPawn   {}", pawn_sum(&grid, col));
        println!("\tBishop {}", bishop_sum(&grid, col));
        println!("\tKnight {}", knight_sum(&grid, col));
        println!("\tKing   {}", king_sum(&grid, col));
    }
}

/// Sums to the last row based on pawn movements.
fn pawn_sum(grid: &Grid, cell: usize) -> usize {
    // If on the last row, return cell value
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    // Else return the sum of the current cell with a recursive call
    grid.board[cell] + pawn_sum(grid, cell + grid.cols)
}

/// Sums to the last row based on bishop movements.
fn bishop_sum(grid: &Grid, cell: usize) -> usize {
    let mut left_sum = grid.board[cell];
    let mut right_sum = grid.board[cell];

    // If on the last row, return cell value
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    // If a left diagonal path is available, get its sum
    if cell % grid.cols != 0 {
        left_sum += bishop_sum(grid, cell + grid.cols - 1);
    }

    // If a right diagonal path is available, get its sum
    if cell % grid.cols != grid.cols - 1 {
        right_sum += bishop_sum(grid, cell + grid.cols + 1);
    }

    // Return the greater sum
    left_sum.max(right_sum)
}

/// Sums to the last row based on knight movements.
fn knight_sum(grid: &Grid, cell: usize) -> usize {
    let mut left_sum = grid.board[cell];
    let mut right_sum = grid.board[cell];

    // If on the last row, return cell value
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    // If a left knight's path is available, get its sum
    if cell % grid.cols > 1 {
        left_sum += knight_sum(grid, cell + grid.cols - 2);
    }

    // If a right knight's path is available, get its sum
    if cell % grid.cols < grid.cols - 2 {
        right_sum += knight_sum(grid, cell + grid.cols + 2);
    }

    // Return the greater sum
    left_sum.max(right_sum)
}

/// Sums to the last row based on king movements.
fn king_sum(grid: &Grid, cell: usize) -> usize {
    let mut down_sum = grid.board[cell];
    let mut left_sum = grid.board[cell];
    let mut right_sum = grid.board[cell];

    // If on the last row, return cell value
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    // Sum the column vertically
    down_sum += king_sum(grid, cell + grid.cols);

    // If a left diagonal path is available, get its sum
    if cell % grid.cols != 0 {
        left_sum += king_sum(grid, cell + grid.cols - 1);
    }

    // If a right diagonal path is available, get its sum
    if cell % grid.cols != grid.cols - 1 {
        right_sum += king_sum(grid, cell + grid.cols + 1);
    }

    // Return the greater sum
    down_sum.max(left_sum.max(right_sum))
}

/// Displays the supplied board.
fn display_board(grid: &Grid) {
    print!("\tBoard:");
    for cell in 0..grid.size {
        if cell % grid.cols == 0 {
            println!();
        }
        print!("{:>0width$} ", grid.board[cell], width = 2);
    }
    println!("\n");
}
