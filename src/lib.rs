mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;

extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for dr in [self.height - 1, 0, 1].iter().cloned() {
            for dc in [self.width - 1, 0, 1].iter().cloned() {
                if dr == 0u32 && dc == 0u32 {
                    continue;
                }

                let nr = (row + self.height + dr) % self.height;
                let nc = (col + self.width + dc) % self.width;
                let idx = self.get_index(nr, nc);
                count += self.cells[idx] as u8;
            }
        }

        count
    }
}

/// Public methods, exported to JS
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // lack of resources
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // stable
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // overcrowding
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // birth
                    (Cell::Dead, 3) => Cell::Alive,
                    // everything else stays the same
                    (otherwise, _) => otherwise, 
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Dead
                } else {
                    Cell::Alive
                }
            })
            .collect();
        Universe {
            width,
            height,
            cells,
        }
    }

    // .#.
    // ..#
    // ###
    pub fn init_spaceship(&mut self) {
        for (r, c) in [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)].iter().cloned() {
            let idx = self.get_index(r, c);
            self.cells[idx] = Cell::Alive;
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
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
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '□' } else { '■' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
