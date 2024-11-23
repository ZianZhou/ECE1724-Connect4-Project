use std::io::{self, Write};
//COLS should not exceed 10, easy to represent in digits
pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const EMPTY: char = '.';
pub const PLAYER_X: char = 'X';
pub const PLAYER_O: char = 'O';
pub const EXPANDED_ROWS: usize = 10;
pub const EXPANDED_COLS: usize = 10;

pub static mut IF_EXPAND: bool = false;


pub struct Game {
    board: Vec<Vec<char>>,
    // board: [[char; COLS]; ROWS],
    current_player: char,
    rows: usize,
    cols: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            // board: [[EMPTY; COLS]; ROWS],
            board: vec![vec![EMPTY; COLS]; ROWS],
            current_player: PLAYER_X,
            rows: ROWS,
            cols: COLS,
        }
    }

    //used in frontend calls
    pub fn get_board(&self) -> &Vec<Vec<char>> {
        &self.board
    }

    pub fn print_board(&self) {
        for row in self.board.iter().rev() {
            for &cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }

    pub fn drop_piece(&mut self, col: usize) -> Result<(), String> {
        if col >= self.cols {
            return Err("Invalid column.".to_string());
        }

        for row in 0..self.rows {
            if self.board[row][col] == EMPTY {
                self.board[row][col] = self.current_player;
                return Ok(());
            }
        }

        Err("Column is full.".to_string())
    }

    pub fn switch_player(&mut self) {
        self.current_player = if self.current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
    }

    pub fn check_winner(&self) -> Option<char> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.board[row][col] != EMPTY {
                    let player = self.board[row][col];
                    //check in three directions
                    if col + 3 < self.cols
                        && self.board[row][col + 1] == player
                        && self.board[row][col + 2] == player
                        && self.board[row][col + 3] == player
                    {
                        return Some(player);
                    }

                    if row + 3 < self.rows
                        && self.board[row + 1][col] == player
                        && self.board[row + 2][col] == player
                        && self.board[row + 3][col] == player
                    {
                        return Some(player);
                    }

                    if row + 3 < self.rows
                        && col + 3 < self.cols
                        && self.board[row + 1][col + 1] == player
                        && self.board[row + 2][col + 2] == player
                        && self.board[row + 3][col + 3] == player
                    {
                        return Some(player);
                    }

                    if row >= 3
                        && col + 3 < self.cols
                        && self.board[row - 1][col + 1] == player
                        && self.board[row - 2][col + 2] == player
                        && self.board[row - 3][col + 3] == player
                    {
                        return Some(player);
                    }
                }
            }
        }
        None
    }

    pub fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell != EMPTY))
    }

    pub fn expand_board(&mut self) {
        println!("Expanding the board to 10x10!");
        self.rows = EXPANDED_ROWS;
        self.cols = EXPANDED_COLS;
        let mut new_board = vec![vec![EMPTY; EXPANDED_COLS]; EXPANDED_ROWS];

        for r in 0..ROWS {
            for c in 0..COLS {
                new_board[r][c] = self.board[r][c];
            }
        }

        self.board = new_board;
    }

    pub fn play(&mut self) {
        loop {
            self.print_board();
            print!(
                "Player {}'s turn. Enter column (0-{}): ",
                self.current_player,
                self.cols - 1
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            //input must be digits between 0-9
            let col = match input.trim().parse::<usize>() {
                Ok(num) if num < self.cols => num,
                _ => {
                    println!(
                        "Invalid input. Please enter a number between 0 and {}.",
                        self.cols - 1
                    );
                    continue;
                }
            };

            //drop the piece
            if let Err(err) = self.drop_piece(col) {
                println!("{}", err);
                continue;
            }

            //Check for a winner or draw
            if let Some(winner) = self.check_winner() {
                self.print_board();
                println!("Player {} wins!", winner);
                break;
            }

            if self.is_full() {
                unsafe {
                    if !IF_EXPAND {
                        IF_EXPAND = true;
                        self.expand_board();
                        continue;
                    }
                }

                self.print_board();
                println!("It's a tie!");
                break;
            }

            //Switch player
            self.switch_player();
        }
    }
}

pub fn backend() {
    let mut game = Game::new();
    game.play();
}
