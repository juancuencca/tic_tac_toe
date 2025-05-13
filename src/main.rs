use std::{fmt, io::{self, Write}};

const ROWS: usize = 3;
const COLS: usize = 3;

fn main() {
    let mut board = Board::new();
    let mut turn = Player::A;

    println!("Welcome to Tic Tac Toe game");
    println!("{} starts the game\n", turn);
    
    loop {
        println!("Turn of {}", turn);

        let fig = match turn {
            Player::A => Figure::Cross,
            Player::B => Figure::Circle,
        };

        let input = match read_input("row") {
            Ok(buf) => buf,
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        };

        let row: usize = match input.trim().parse() {
            Ok(row) => row,
            Err(error) => {
                println!("Could not parse input to a valid usize value: {}", error);
                continue;
            }
        };
        
        let input = match read_input("col") {
            Ok(buf) => buf,
            Err(error) => {
                println!("error: {error}");
                continue;
            }
        };

        let col: usize = match input.trim().parse() {
            Ok(col) => col,
            Err(error) => {
                println!("Could not parse input to a valid usize value: {}", error);
                continue;
            }
        }; 

        if row > 2 || col > 2 {
            println!("row and col must be in the range [0-2]");
            continue;
        }

        let marked_count = match board.mark_cell(fig, row, col) {
            Some(marked) => marked,
            None => {
                println!("Cell already marked");
                continue;
            }
        };

        board.show_board();

        if board.complete_line() {
            println!("{} won. Congratulations!", turn);
            break;
        }

        if marked_count == ROWS * COLS {
            println!("The game ended in a draw");
            break;
        } 

        turn = turn.change();
    }
}

fn read_input(axis: &str) -> io::Result<String> {
    print!("Enter the {}: ", axis);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(buf)
}

struct Board {
    cells: Vec<Option<Figure>>,
    marked_count: usize,
}

impl Board {
    fn new() -> Board {
        Board {
            cells: (0..ROWS * COLS).map(|_| None).collect(),
            marked_count: 0,
        }
    }

    fn cell_is_empty(&self, row: usize, col: usize) -> bool {
        self.get_cell(row, col).is_none()
    }

    fn complete_line(&self) -> bool {
        for i in 0..ROWS {
            let mut count_circle = 0;
            let mut count_cross = 0;
            for j in 0..COLS {
                match self.get_cell(i, j) {
                    Some(Figure::Circle) => count_circle += 1,
                    Some(Figure::Cross) => count_cross += 1,
                    None => (),
                }
                if count_circle == 3 || count_cross == 3 {
                    return true;
                }
            }
        } 

        for j in 0..COLS {
            let mut count_circle = 0;
            let mut count_cross = 0;
            for i in 0..ROWS {
                match self.get_cell(i, j) {
                    Some(Figure::Circle) => count_circle += 1,
                    Some(Figure::Cross) => count_cross += 1,
                    None => (),
                };
                if count_circle == 3 || count_cross == 3 {
                    return true;
                }
            }
        } 
        
        let mut count_circle = 0;
        let mut count_cross = 0;
        for i in 0..ROWS {
            match self.get_cell(i, i) {
                Some(Figure::Circle) => count_circle += 1,
                Some(Figure::Cross) => count_cross += 1,
                None => (),
            };
            if count_circle == 3 || count_cross == 3 {
                return true;
            } 
        }

        let mut j = 3;
        let mut count_circle = 0;
        let mut count_cross = 0;
        for i in 0..ROWS {
            j -= 1;
            match self.get_cell(i, j) {
                Some(Figure::Circle) => count_circle += 1,
                Some(Figure::Cross) => count_cross += 1,
                None => (),
            };
            if count_circle == 3 || count_cross == 3 {
                return true;
            } 
        }

        false
    }

    fn mark_cell(&mut self, fig: Figure, row: usize, col: usize) -> Option<usize> {
        if !self.cell_is_empty(row, col) {
            return None;
        }

        let position = self.get_position(row, col);
        self.cells[position] = Some(fig);
        self.marked_count += 1;

        Some(self.marked_count)
    }

    fn show_board(&self) {
        println!("-------------------");
        for i in 0..ROWS {
            for j in 0..COLS {
                print!("|  ");
                match self.get_cell(i, j) {
                    None => print!(" "),
                    Some(Figure::Circle) => print!("o"),
                    Some(Figure::Cross) => print!("x"),
                }
                print!("  ");
                if j == COLS - 1 {
                    print!("|");
                }
            }
            println!("\n-------------------");
        }
    }

    fn get_position(&self, row: usize, col: usize) -> usize {
        row * COLS + col
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<&Figure> {
        self.cells[self.get_position(row, col)].as_ref()
    }
}

#[derive(Debug)]
enum Figure {
    Circle,
    Cross,
}

impl fmt::Display for Figure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Figure::Circle => write!(f, "o"),
            Figure::Cross => write!(f, "x"),
        }
    } 
}

enum Player {
    A,
    B,
}

impl Player {
    fn change(self) -> Player {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::A => write!(f, "Player A"),
            Player::B => write!(f, "Player B"),
        }
    } 
}
