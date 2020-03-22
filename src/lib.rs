mod utils;

use wasm_bindgen::prelude::*;

extern crate fixedbitset;
use fixedbitset::FixedBitSet;

extern crate js_sys;

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

                next.set(idx, next_cell)
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width: u32 = 64;
        let height: u32 = 64;
        let total_cells = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(total_cells);

        for idx in (0 as usize)..total_cells {
            // let rand = js_sys::Math::random();
            cells.set(idx, idx % 2 == 0 || idx % 7 == 0);
        }

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
            self.cells.put(idx);
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn get_alive_cells(&self) -> Vec<usize> {
        self.cells.ones().collect()
    }
}
