mod utils;


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().clone() {
            for delta_col in [self.width - 1, 0, 1].iter().clone() {
                if *delta_row == 0 && *delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8
            }
        }
        return count;
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let idx = self.get_index(row, column);
                let cell = self.cells[idx];
                let live_neighbor = self.live_neighbor_count(row, column);
                let next_cell = match (cell, live_neighbor) {
                    // Rule 1: Any live cell with fower than two live neighbors dies because of under population
                    (Cell::Alive, x) if x < 2  => Cell::Dead,
                    //Rule 2: Any live cell with 2 or 3 live neighbors lives on
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than 3 live neighbors dies bc of overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbors becomes a live cell due to reproduction
                    (Cell::Dead, 3) => Cell::Alive,
                    // Otherwise
                    (otherwise, _) => otherwise
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..height*width).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Universe { width, height, cells }
    }

    pub fn render(&self) -> String  {
        self.to_string()
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {"[ ]"} else {"[x]"};
            write!(f, "{}", symbol)?;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
