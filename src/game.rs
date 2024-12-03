use rand::random;
use std::io::{self, Write};

pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const EMPTY: char = '.';
pub const PLAYER_X: char = 'X';
pub const PLAYER_O: char = 'O';
pub const EXPANDED_ROWS: usize = 10;
pub const EXPANDED_COLS: usize = 10;
pub const OBSTACLE: char = '#';

pub struct Game {
    board: Vec<Vec<char>>,
    current_player: char,
    skip_turn: bool,
    rows: usize,
    cols: usize,
    pub expanded: bool,
    pub power_ups_enabled: bool,
}

impl Game {
    pub fn new(power_ups_enabled: bool) -> Game {
        let mut game = Game {
            board: vec![vec![EMPTY; COLS]; ROWS],
            current_player: PLAYER_X,
            skip_turn: false,
            rows: ROWS,
            cols: COLS,
            expanded: false,
            power_ups_enabled,
        };
        if power_ups_enabled {
            game.initialize_power_ups(6);
        }
        game
    }

    pub fn initialize_power_ups(&mut self, num_power_ups: usize) {
        if !self.power_ups_enabled {
            return;
        }
        let mut placed = 0;
        let power_up_types = ['B', 'S', 'H'];

        while placed < num_power_ups {
            let row = random::<usize>() % self.rows;
            let col = random::<usize>() % self.cols;

            if self.board[row][col] == EMPTY {
                let power_up = power_up_types[placed % power_up_types.len()];
                self.board[row][col] = power_up;
                placed += 1;
            }
        }
    }

    pub fn initialize_new_power_ups(&mut self, num_power_ups: usize) {
        if !self.power_ups_enabled {
            return;
        }
        let mut placed = 0;
        let power_up_types = ['B', 'S', 'H'];

        while placed < num_power_ups {
            let row = ROWS + (random::<usize>() % (EXPANDED_ROWS - ROWS));
            let col = random::<usize>() % self.cols;

            if col == self.cols - 2 {
                continue;
            }

            if self.board[row][col] == EMPTY {
                let power_up = power_up_types[placed % power_up_types.len()];
                self.board[row][col] = power_up;
                placed += 1;
            }
        }
    }

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

        for row in 0..self.rows {
            if self.board[row][col] == EMPTY || ['B', 'S', 'H'].contains(&self.board[row][col]) {
                if ['B', 'S', 'H'].contains(&self.board[row][col]) {
                    if self.board[row][col] == 'B' {
                        self.activate_power_up(row, col);
                        return Ok((row, col));
                    }
                    self.activate_power_up(row, col);
                }

                self.board[row][col] = self.current_player;
                return Ok((row, col));
            }

            if self.board[row][col] == OBSTACLE {
                if row + 1 < self.rows && self.board[row + 1][col] == EMPTY {
                    self.board[row + 1][col] = self.current_player;
                    return Ok((row, col));
                }
            }
        }

        Err("Column is full.".to_string())
    }

    pub fn switch_player(&mut self) {
        if self.skip_turn {
            self.skip_turn = false;
        } else {
            self.current_player = if self.current_player == PLAYER_X {
                PLAYER_O
            } else {
                PLAYER_X
            };
        }
    }

    pub fn activate_power_up(&mut self, row: usize, col: usize) {
        if !self.power_ups_enabled {
            return;
        }
        match self.board[row][col] {
            'B' => {
                self.use_bomb(row, col);
            }
            'S' => {
                self.skip_turn = true;
            }
            'H' => {
                self.place_obstacles(row, col);
            }
            _ => {}
        }
    }

    pub fn place_obstacles(&mut self, row: usize, col: usize) {
        let deltas = [(0, -1), (0, 1)];
        for (dr, dc) in deltas.iter() {
            let nr = row as isize + dr;
            let nc = col as isize + dc;

            if nr >= 0 && nr < self.rows as isize && nc >= 0 && nc < self.cols as isize {
                let mut target_row = nr as usize;
                let target_col = nc as usize;

                while target_row > 0
                    && (self.board[target_row - 1][target_col] == EMPTY
                        || ['B', 'S', 'H'].contains(&self.board[target_row - 1][target_col]))
                {
                    target_row -= 1;
                }

                self.board[target_row][target_col] = OBSTACLE;
            }
        }
    }

    pub fn use_bomb(&mut self, row: usize, col: usize) {
        if row > 0 {
            self.board[row][col] = EMPTY;
            self.board[row - 1][col] = EMPTY;
        } else if row == 0 {
            self.board[0][col] = EMPTY;
        }
        self.skip_turn = true;
    }

    pub fn check_winner(&self) -> Option<char> {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.board[row][col] != EMPTY {
                    let player = self.board[row][col];
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
        self.rows = EXPANDED_ROWS;
        self.cols = EXPANDED_COLS;
        let mut new_board = vec![vec![EMPTY; EXPANDED_COLS]; EXPANDED_ROWS];

        for r in 0..ROWS {
            for c in 0..COLS {
                new_board[r][c] = self.board[r][c];
            }
        }

        self.board = new_board;
        let num_new_power_ups = if self.power_ups_enabled {
            (EXPANDED_ROWS * EXPANDED_COLS - ROWS * COLS) / 10
        } else {
            0
        };
        self.initialize_new_power_ups(num_new_power_ups);
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

            if let Err(err) = self.drop_piece(col) {
                println!("{}", err);
                continue;
            }

            if let Some(winner) = self.check_winner() {
                self.print_board();
                println!("Player {} wins!", winner);
                break;
            }

            if self.is_full() {
                if !self.expanded {
                    self.expand_board();
                    self.expanded = true;
                    continue;
                }

                self.print_board();
                println!("It's a tie!");
                break;
            }

            self.switch_player();
        }
    }
}

pub fn backend() {
    let mut game = Game::new(true);
    game.play();
}
