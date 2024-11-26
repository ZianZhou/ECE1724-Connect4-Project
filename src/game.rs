use rand::random;
use std::io::{self, Write};

//COLS should not exceed 10, easy to represent in digits
pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const EMPTY: char = '.';
pub const PLAYER_X: char = 'X';
pub const PLAYER_O: char = 'O';
pub const EXPANDED_ROWS: usize = 10;
pub const EXPANDED_COLS: usize = 10;

// Power Up: Obstacle
pub const OBSTACLE: char = '#'; // # represent the obstacle on the map

pub static mut IF_EXPAND: bool = false;

pub struct Game {
    board: Vec<Vec<char>>,
    // board: [[char; COLS]; ROWS],
    current_player: char,
    skip_turn: bool,
    rows: usize,
    cols: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            // board: [[EMPTY; COLS]; ROWS],
            board: vec![vec![EMPTY; COLS]; ROWS],
            current_player: PLAYER_X,
            skip_turn: false,
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

    pub fn get_current_player(&self) -> char {
        self.current_player
    }

    pub fn drop_piece(&mut self, col: usize) -> Result<(usize, usize), String> {
        if col >= self.cols {
            return Err("Invalid column.".to_string());
        }

        // Iterate through rows in the column
        for row in 0..self.rows {
            // Allow placement on empty cells or Power-Ups
            if self.board[row][col] == EMPTY || self.board[row][col] == 'P' {
                // Ensure no piece is placed below an obstacle
                if row > 0 && self.board[row - 1][col] == OBSTACLE && self.board[row][col] != 'P' {
                    return Err("Cannot place a piece below an obstacle.".to_string());
                }

                // Trigger Power-Up effect if applicable
                if self.board[row][col] == 'P' {
                    println!("Power-Up activated!");
                    self.activate_power_up(row, col);
                }

                self.board[row][col] = self.current_player; // Place the piece
                return Ok((row, col));
            }

            // Allow placement directly above an obstacle
            if self.board[row][col] == OBSTACLE {
                // Ensure this is the highest valid position
                if row + 1 < self.rows && self.board[row + 1][col] == EMPTY {
                    self.board[row + 1][col] = self.current_player; // Place above the obstacle
                    return Ok((row, col));
                }
            }
        }

        Err("Column is full.".to_string())
    }

    pub fn switch_player(&mut self) {
        if self.skip_turn {
            self.skip_turn = false; // represent the bool of skipping turns
        } else {
            self.current_player = if self.current_player == PLAYER_X {
                PLAYER_O
            } else {
                PLAYER_X
            };
        }
    }

    pub fn generate_power_up(&mut self) {
        for _ in 0..10 {
            // choose a col randomly
            let col = random::<usize>() % self.cols;
            for row in 0..self.rows {
                if self.board[row][col] == EMPTY {
                    // Make sure Power-Up can only be placed at the bottom
                    if row == 0 || self.board[row - 1][col] != EMPTY {
                        self.board[row][col] = 'P'; // P represent the Power Up
                        return;
                    }
                }
            }
        }
    }

    pub fn cleanup_power_ups(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.board[row][col] == 'P' {
                    self.board[row][col] = EMPTY;
                }
            }
        }
    }

    pub fn activate_power_up(&mut self, row: usize, col: usize) {
        // random choose a Power Up element
        match rand::random::<u8>() % 3 {
            0 => {
                println!("Power-Up activated: Skip opponent's turn!");
                self.skip_turn = true;
            }
            1 => {
                println!("Power-Up activated: Place obstacles!");
                self.place_obstacles(row, col);
            }
            2 => {
                println!("Power-Up activated: Bomb triggered!");
                self.use_bomb(row, col);
            }
            _ => {}
        }
    }

    // Power Up for Placing Obstacles
    pub fn place_obstacles(&mut self, row: usize, col: usize) {
        let deltas = [(0, -1), (0, 1)]; // Only consider left and right directions
        for (dr, dc) in deltas.iter() {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            // Ensure the position is within bounds
            if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                let target_row = nr as usize;
                let target_col = nc as usize;

                // Place obstacle only if the cell is empty or occupied by the opponent's piece
                if self.board[target_row][target_col] == EMPTY
                    || self.board[target_row][target_col] != self.current_player
                {
                    self.board[target_row][target_col] = OBSTACLE; // Place the obstacle
                }
            }
        }
    }

    // Power Up for Bombs (Clear the pieces between)
    pub fn use_bomb(&mut self, row: usize, col: usize) {
        let deltas = [(0, -1), (0, 1)];
        for (dr, dc) in deltas.iter() {
            let nr = row as isize + dr;
            let nc = col as isize + dc;
            if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                self.board[nr as usize][nc as usize] = EMPTY;
            }
        }
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
            // clean up untriggered Power Up
            self.cleanup_power_ups();
            // 20% percent possibility of generating Power Up
            if rand::random::<u8>() % 5 == 0 {
                self.generate_power_up();
            }

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
