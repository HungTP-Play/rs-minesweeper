use rand::{thread_rng, Rng};

#[derive(Debug)]
struct Minesweeper {
    board: Vec<Vec<u8>>,
    flagged: Vec<Vec<bool>>,
    revealed: Vec<Vec<bool>>,
    game_over: bool,
    number_of_mines: u8,
    win: bool,
    cursor: (u8, u8),
}

impl Minesweeper {
    // Create a new Minesweeper game
    fn new(width: u8, height: u8, number_of_mines: u8) -> Minesweeper {
        let mut board = vec![vec![0; width as usize]; height as usize];
        let flagged = vec![vec![false; width as usize]; height as usize];
        let revealed = vec![vec![false; width as usize]; height as usize];
        let mut rng = thread_rng();
        let mut mines = 0;

        // Place mines randomly
        while mines < number_of_mines {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            if board[y as usize][x as usize] != 9 {
                board[y as usize][x as usize] = 9;
                mines += 1;
            }
        }

        // Count adjacent mines
        for y in 0..height {
            for x in 0..width {
                if board[y as usize][x as usize] == 9 {
                    // Top left
                    if x > 0 && y > 0 && board[(y - 1) as usize][(x - 1) as usize] != 9 {
                        board[(y - 1) as usize][(x - 1) as usize] += 1;
                    }
                    // Top
                    if y > 0 && board[(y - 1) as usize][x as usize] != 9 {
                        board[(y - 1) as usize][x as usize] += 1;
                    }
                    // Top right
                    if x < width - 1 && y > 0 && board[(y - 1) as usize][(x + 1) as usize] != 9 {
                        board[(y - 1) as usize][(x + 1) as usize] += 1;
                    }
                    // Left
                    if x > 0 && board[y as usize][(x - 1) as usize] != 9 {
                        board[y as usize][(x - 1) as usize] += 1;
                    }
                    // Right
                    if x < width - 1 && board[y as usize][(x + 1) as usize] != 9 {
                        board[y as usize][(x + 1) as usize] += 1;
                    }
                    // Bottom left
                    if x > 0 && y < height - 1 && board[(y + 1) as usize][(x - 1) as usize] != 9 {
                        board[(y + 1) as usize][(x - 1) as usize] += 1;
                    }
                    // Bottom
                    if y < height - 1 && board[(y + 1) as usize][x as usize] != 9 {
                        board[(y + 1) as usize][x as usize] += 1;
                    }
                    // Bottom right
                    if x < width - 1
                        && y < height - 1
                        && board[(y + 1) as usize][(x + 1) as usize] != 9
                    {
                        board[(y + 1) as usize][(x + 1) as usize] += 1;
                    }
                }
            }
        }

        Minesweeper {
            board,
            flagged,
            revealed,
            game_over: false,
            number_of_mines,
            win: false,
            cursor: (0, 0),
        }
    }

    // Reveal a cell
    fn reveal(&mut self, x: u8, y: u8) {
        if !self.flagged[y as usize][x as usize] {
            if !self.revealed[y as usize][x as usize] == true {
                self.revealed[y as usize][x as usize] = true;
                if self.board[y as usize][x as usize] == 9 {
                    self.game_over = true;
                }
    
                // Reveal adjacent cells if cell is empty
                if self.board[y as usize][x as usize] == 0 {
                    // Top left
                    if x > 0 && y > 0 {
                        self.reveal(x - 1, y - 1);
                    }

                    // Top
                    if y > 0 {
                        self.reveal(x, y - 1);
                    }

                    // Top right
                    if x < self.board[0].len() as u8 - 1 && y > 0 {
                        self.reveal(x + 1, y - 1);
                    }

                    // Left
                    if x > 0 {
                        self.reveal(x - 1, y);
                    }

                    // Right
                    if x < self.board[0].len() as u8 - 1 {
                        self.reveal(x + 1, y);
                    }

                    // Bottom left
                    if x > 0 && y < self.board.len() as u8 - 1 {
                        self.reveal(x - 1, y + 1);
                    }

                    // Bottom
                    if y < self.board.len() as u8 - 1 {
                        self.reveal(x, y + 1);
                    }

                    // Bottom right
                    if x < self.board[0].len() as u8 - 1 && y < self.board.len() as u8 - 1 {
                        self.reveal(x + 1, y + 1);
                    }
                }
            }
        }
    }

    // Toggle a flag
    fn toggle_flag(&mut self, x: u8, y: u8) {
        if !self.revealed[y as usize][x as usize] {
            self.flagged[y as usize][x as usize] = !self.flagged[y as usize][x as usize];
        }

        // Check win -> All mine cells are flagged
        let mut flagged_mines = 0;
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                if self.board[y][x] == 9 && self.flagged[y][x] {
                    flagged_mines += 1;
                }
            }
        }

        if flagged_mines == self.number_of_mines {
            self.win = true;
            self.game_over = true;
        }
    }

    fn move_cursor(&mut self, x: u8, y: u8) {
        self.cursor = (x, y);
    }

}

fn main() {
    // Initialize game
    let mut minesweeper = Minesweeper::new(10, 10, 10);

    // Reveal cell (0, 0)
    minesweeper.reveal(0, 0);

    // Toggle flag on cell (0, 1)
    minesweeper.toggle_flag(0, 1);

    // Move cursor to (5, 5)
    minesweeper.move_cursor(5, 5);
    
    // Print board
    println!("{:?}", minesweeper.board);
}
