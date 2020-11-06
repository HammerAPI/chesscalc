use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

struct Grid {
    board: Vec<usize>,
    cols: usize,
    _rows: usize,
    size: usize,
}
impl Grid {
    fn new(filename: &str) -> Self {
        let contents = read_to_string(filename).expect("Could not find file");

        let board: Vec<usize> = contents
            .split_whitespace()
            .filter(|c| c.parse::<usize>().is_ok())
            .map(|s| s.parse().unwrap())
            .collect();

        let cols = (board.len() as f32).sqrt() as usize;
        let _rows = cols;
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

fn pawn_sum(grid: &Grid, cell: usize) -> usize {
    // If on last row
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    grid.board[cell] + pawn_sum(grid, cell + grid.cols)
}

fn bishop_sum(grid: &Grid, cell: usize) -> usize {
    let mut left_sum = grid.board[cell];
    let mut right_sum = grid.board[cell];

    // If on last row
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    if cell % grid.cols != 0 {
        left_sum += bishop_sum(grid, cell + grid.cols - 1);
    }

    if cell % grid.cols != grid.cols - 1 {
        right_sum += bishop_sum(grid, cell + grid.cols + 1);
    }

    left_sum.max(right_sum)
}

fn knight_sum(grid: &Grid, cell: usize) -> usize {
    let mut left_sum = grid.board[cell];
    let mut right_sum = grid.board[cell];

    // If on last row
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    if cell % grid.cols > 1 {
        left_sum += knight_sum(grid, cell + grid.cols - 2);
    }

    if cell % grid.cols < grid.cols - 2 {
        right_sum += knight_sum(grid, cell + grid.cols + 2);
    }

    left_sum.max(right_sum)
}

fn king_sum(grid: &Grid, cell: usize) -> usize {
    let mut down_sum = grid.board[cell];
    let mut left_sum = grid.board[cell];
    let mut right_sum = grid.board[cell];

    // If on last row
    if cell + grid.cols >= grid.size {
        return grid.board[cell];
    }

    down_sum += king_sum(grid, cell + grid.cols);

    if cell % grid.cols != 0 {
        left_sum += king_sum(grid, cell + grid.cols - 1);
    }

    if cell % grid.cols != grid.cols - 1 {
        right_sum += king_sum(grid, cell + grid.cols + 1);
    }

    down_sum.max(left_sum.max(right_sum))
}

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
