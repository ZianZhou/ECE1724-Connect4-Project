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
        let mut game =Game {
            // board: [[EMPTY; COLS]; ROWS],
            board: vec![vec![EMPTY; COLS]; ROWS],
            current_player: PLAYER_X,
            skip_turn: false,
            rows: ROWS,
            cols: COLS,
        };
        // Initialize Power-Ups during game creation
        game.initialize_power_ups(6); // Place 6 Power-Ups on the board
        game
    }

    /// Initialize Power-Ups on the board
    pub fn initialize_power_ups(&mut self, num_power_ups: usize) {
        let mut placed = 0;
        let power_up_types = ['B', 'S', 'H']; // Bomb, Skip, and Obstacle

        while placed < num_power_ups {
            // Randomly choose a row and column
            let row = random::<usize>() % self.rows;
            let col = random::<usize>() % self.cols;

            // Place Power-Up only if the cell is empty
            if self.board[row][col] == EMPTY {
                let power_up = power_up_types[placed % power_up_types.len()];
                self.board[row][col] = power_up; // Place the Power-Up
                placed += 1;
            }
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
            if self.board[row][col] == EMPTY || ['B', 'S', 'H'].contains(&self.board[row][col]) {
                // Ensure no piece is placed below an obstacle
                if row > 0 && self.board[row - 1][col] == OBSTACLE && self.board[row][col] != EMPTY {
                    return Err("Cannot place a piece below an obstacle.".to_string());
                }

                // Trigger Power-Up effect if applicable
                if ['B', 'S', 'H'].contains(&self.board[row][col]) {
                    println!("Power-Up activated!");
                    if self.board[row][col]=='B'{
                        self.activate_power_up(row, col);
                        return Ok((row, col));
                    }
                    self.activate_power_up(row, col);

                }

                // Place the piece
                self.board[row][col] = self.current_player;
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


    pub fn activate_power_up(&mut self, row: usize, col: usize) {
        match self.board[row][col] {
            'B' => {
                println!("Power-Up activated: Bomb triggered!");
                self.use_bomb(row, col);
            }
            'S' => {
                println!("Power-Up activated: Skip opponent's turn!");
                self.skip_turn = true;
            }
            'H' => {
                println!("Power-Up activated: Place obstacles!");
                self.place_obstacles(row, col);
            }
            _ => {}
        }
    }

    // Power Up for Placing Obstacles
    // Power Up for Placing Obstacles
    pub fn place_obstacles(&mut self, row: usize, col: usize) {
        let deltas = [(0, -1), (0, 1)]; // Only consider left and right directions
        for (dr, dc) in deltas.iter() {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            // Ensure the position is within bounds
            if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                let mut target_row = nr as usize;
                let target_col = nc as usize;

                // Let the obstacle fall to the bottom or until it meets another piece
                while target_row > 0 && self.board[target_row - 1][target_col] == EMPTY {
                    target_row -= 1;
                }

                // Place obstacle
                self.board[target_row][target_col] = OBSTACLE;
            }
        }
    }

    // Power Up for Bombs (Clear the pieces between)
    pub fn use_bomb(&mut self, row: usize, col: usize) {
        if row > 0 {
            // Clear the piece below the bomb
            self.board[row - 1][col] = self.current_player;
            // Drop the piece above to the bomb's position
            self.board[row][col] = EMPTY;
        }
        else if row==0{
            self.board[0][col] = self.current_player;
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
        // Add Power-Ups only to new empty spaces
        let num_new_power_ups = (EXPANDED_ROWS * EXPANDED_COLS - ROWS * COLS) / 10; // 10% of new cells
        self.initialize_new_power_ups(num_new_power_ups);
    }

    /// Add Power-Ups to new empty spaces only after board expansion
    pub fn initialize_new_power_ups(&mut self, num_power_ups: usize) {
        let mut placed = 0;
        let power_up_types = ['B', 'S', 'H']; // Bomb, Skip, and Obstacle

        while placed < num_power_ups {
            // Randomly choose a row and column within the new rows
            let row = ROWS + random::<usize>() % (EXPANDED_ROWS - ROWS);
            let col = random::<usize>() % self.cols;

            // Exclude the second-to-last column
            if col == self.cols - 2 {
                continue;
            }

            // Place Power-Up only if the cell is empty
            if self.board[row][col] == EMPTY {
                let power_up = power_up_types[placed % power_up_types.len()];
                self.board[row][col] = power_up; // Place the Power-Up
                placed += 1;
            }
        }
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
