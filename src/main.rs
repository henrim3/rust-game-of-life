use std::io;
use std::io::Write;
use raylib::prelude::*;
use rand::Rng;

#[derive(Clone, Debug)]
enum CellStatus {
    Alive,
    Dead,
}

impl std::fmt::Display for CellStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CellStatus::Alive => write!(f, "#"),
            CellStatus::Dead => write!(f, " "),
        }
    }
}

struct Board {
    width: usize,
    height: usize,
    grid: Vec<Vec<CellStatus>>,
}

impl Board {
    // create a new [Board] with width and height, cell size in pixels
    fn new(width: usize, height: usize) -> Board {
        Board { 
            width: width, 
            height: height, 
            grid: vec![vec![CellStatus::Dead; width]; height], 
        }
    }

    fn get_num_alive_neighbors(&self, row: i32, col: i32) -> i32 {
        let mut num_alive_neighbors: i32 = 0;
        for r_off in -1..2 {
            for c_off in -1..2 {
                if r_off == 0 && c_off == 0 {
                    continue;
                }

                let r = row + r_off;
                let c = col + c_off;

                if r < 0 || c < 0 || r >= self.height as i32 || c >= self.width as i32 {
                    continue;
                }

                match &self.grid[r as usize][c as usize] {
                    CellStatus::Alive => num_alive_neighbors += 1,
                    _ => (),
                }
            }
        }
        return num_alive_neighbors;
    }

    fn run_turn(&mut self) {
        let mut next_grid = vec![vec![CellStatus::Dead; self.width]; self.height];

        for r in 0..self.height {
            for c in 0..self.width {
                let cell: &CellStatus = &self.grid[r][c];
                let num_neighbors = self.get_num_alive_neighbors(r as i32, c as i32);
                match cell {
                    CellStatus::Alive => {
                        if num_neighbors == 2 || num_neighbors == 3 {
                            next_grid[r][c] = CellStatus::Alive;
                        }
                    },
                    CellStatus::Dead => {
                        if num_neighbors == 3 {
                            next_grid[r][c] = CellStatus::Alive;
                        }
                    },
                }
            }
        }

        self.grid = next_grid;
    }

    // output grid to console using *'s to represent alive cells
    fn console_draw(&self) {
        for row in &self.grid {
            for cell in row {
                print!("{} ", cell);
                io::stdout().flush().unwrap();
            }
            println!("");
        }
    }

    // output grid to Raylib window
    fn window_draw(&self, d: &mut RaylibDrawHandle<'_>, screen_width: i32, screen_height: i32) {
        // calculate cell sizes in window
        let cell_width: f32 = screen_width as f32 / self.width as f32;
        let cell_height: f32 = screen_height as f32 / self.height as f32;

        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        for row in &self.grid {
            for cell in row {
                match cell {
                    CellStatus::Alive => d.draw_rectangle(x as i32, y as i32, cell_width.round() as i32, cell_height.round() as i32, Color::BLACK),
                    CellStatus::Dead => d.draw_rectangle(x as i32, y as i32, cell_width.round() as i32, cell_height.round() as i32, Color::WHITE),
                }

                x += cell_width;
            }
            x = 0.0;
            y += cell_height;
        }
    }

    // randomize grid cells having a [chance] percentage chance of being alive
    // chance can be 0-100
    fn randomize(&mut self, chance: i32) {
        for row in &mut self.grid {
            for cell in row {
                let rand: bool = rand::thread_rng().gen_range(1..101) <= chance;
                match rand {
                    true => *cell = CellStatus::Alive,
                    false => *cell = CellStatus::Dead,
                } 
            }
        }
    }
}

// params
const SCREEN_WIDTH: i32 = 1080;
const SCREEN_HEIGHT: i32 = 1080;
const BOARD_WIDTH: usize = 500;
const BOARD_HEIGHT: usize = 500;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Game of Life")
        .build();

    rl.set_target_fps(60);

    let mut board: Board = Board::new(BOARD_WIDTH, BOARD_HEIGHT);
    board.randomize(40);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        board.window_draw(&mut d, SCREEN_WIDTH, SCREEN_HEIGHT);
        board.run_turn();
    }
}
