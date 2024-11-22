use std::io::{self, Write};
//COLS should not exceed 10, easy to represent in digits
pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const EMPTY: char = '.';
pub const PLAYER_X: char = 'X';
pub const PLAYER_O: char = 'O';

pub struct Game {
    board: [[char; COLS]; ROWS],
    current_player: char,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[EMPTY; COLS]; ROWS],
            current_player: PLAYER_X,
        }
    }

    pub fn get_board(&self) -> &[[char; COLS]; ROWS] {
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
        println!("Go in drop_piece");
        if col >= COLS {
            return Err("Invalid column.".to_string());
        }

        for row in 0..ROWS {
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
        for row in 0..ROWS {
            for col in 0..COLS {
                if self.board[row][col] != EMPTY {
                    let player = self.board[row][col];
                    //check in three directions
                    if col + 3 < COLS
                        && self.board[row][col + 1] == player
                        && self.board[row][col + 2] == player
                        && self.board[row][col + 3] == player
                    {
                        return Some(player);
                    }

                    if row + 3 < ROWS
                        && self.board[row + 1][col] == player
                        && self.board[row + 2][col] == player
                        && self.board[row + 3][col] == player
                    {
                        return Some(player);
                    }

                    if row + 3 < ROWS
                        && col + 3 < COLS
                        && self.board[row + 1][col + 1] == player
                        && self.board[row + 2][col + 2] == player
                        && self.board[row + 3][col + 3] == player
                    {
                        return Some(player);
                    }

                    if row >= 3
                        && col + 3 < COLS
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

    pub fn play(&mut self) {
        loop {
            self.print_board();
            print!(
                "Player {}'s turn. Enter column (0-6): ",
                self.current_player
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            //input must be digits between 0-9
            let col = match input.trim().parse::<usize>() {
                Ok(num) if num < COLS => num,
                _ => {
                    //Todo: after grid expansion, update the column number
                    println!("Invalid input. Please enter a number between 0 and 6.");
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
