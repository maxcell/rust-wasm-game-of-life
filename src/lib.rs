mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)] // Come back to this
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Inactive = 0,
    Active = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

// Blog posts
// Look into std::ops::Index
// Universe[row,col]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8;
            }
        }
        count
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "beef")
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Inactive { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbor_count = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbor_count) {
                    (Cell::Active, x) if x < 2 => Cell::Inactive,
                    (Cell::Active, 2) | (Cell::Active, 3) => Cell::Active,
                    (Cell::Active, x) if x > 3 => Cell::Inactive,
                    (Cell::Inactive, 3) => Cell::Active,
                    (otherwise, _) => otherwise,
                };

                // Next set of cells
                next[index] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Active
                } else {
                    Cell::Inactive
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

// Look at this later

// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);

//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: i32);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, wasm-game-of-life!");
// }

// #[wasm_bindgen]
// pub fn wave(s: &str) {
//     alert(&format!("Hello, {}", s));
// }

// #[wasm_bindgen]
// pub fn add(a: i32, b:i32) {
//     let result = a + b;
//     log(result);
// }
