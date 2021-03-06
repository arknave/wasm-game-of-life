mod utils;

use wasm_bindgen::prelude::*;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

extern crate js_sys;
extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

/// Private methods for Rust
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

    fn total_cells(&self) -> usize {
        (self.width * self.height) as usize
    }
}

/// Public methods for Rust only (testing)
impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        self.cells.clear();
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.put(idx);
        }
    }
}

const SPACESHIP: [(u32, u32); 5] = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];

// For the pulsar, we just hard-code one of the four quadrants
const QUADRANTS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const PULSAR: [(i32, i32); 12] = [(2, 1), (3, 1), (4, 1), (1, 2), (1, 3), (1, 4), (2, 6), (3, 6), (4, 6), (6, 2), (6, 3), (6, 4)];

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

                /*
                log!(
                    "cell({}, {}) was {} and has {} live neighbors!",
                    row,
                    col,
                    cell,
                    live_neighbors,
                );
                */

                let next_cell = match (cell, live_neighbors) {
                    // lack of resources
                    (true, x) if x < 2 => false,
                    // stable
                    (true, 2) | (true, 3) => true,
                    // overcrowding
                    (true, x) if x > 3 => false,
                    // birth
                    (false, 3) => true,
                    // everything else stays the same
                    (otherwise, _) => otherwise, 
                };

                /*
                log!(
                    "cell is now {}",
                    next_cell,
                );
                */

                next.set(idx, next_cell)
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width: u32 = 64;
        let height: u32 = 64;
        let total_cells = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(total_cells);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.reset_cells();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.reset_cells();
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn get_alive_cells(&self) -> Vec<usize> {
        self.cells.ones().collect()
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
    }

    pub fn random_cells(&mut self) {
        for idx in (0 as usize)..self.total_cells() {
            let rand = js_sys::Math::random();
            self.cells.set(idx, rand < 0.5);
        }
    }

    pub fn reset_cells(&mut self) {
        self.cells.grow(self.total_cells());
        self.cells.clear();
    }

    pub fn add_spaceship(&mut self, row: u32, col: u32) {
        for (dr, dc) in SPACESHIP.iter().cloned() {
            let r = (row + dr) % self.height;
            let c = (col + dc) % self.width;
            let idx = self.get_index(r, c);
            self.cells.put(idx);
        }
    }

    pub fn add_pulsar(&mut self, row: i32, col: i32) {
        // I'm not proud of this :-(
        let w = self.width as i32;
        let h = self.height as i32;
        for (sr, sc) in QUADRANTS.iter().cloned() {
            for (dr, dc) in PULSAR.iter().cloned() {
                let r = (h + row + sr * dr) % h;
                let c = (w + col + sc * dc) % w;
                let idx = self.get_index(r as u32, c as u32);
                self.cells.put(idx);
            }
        }
    }
}
