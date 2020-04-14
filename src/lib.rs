extern crate fixedbitset;
extern crate web_sys;
mod util;
use fixedbitset::FixedBitSet;
use util::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// It is IMPORTANT that we have #[repr(u8)],
// so that each cell is represented as a single byte
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// The universe has a width and a height, and
// a vector of cells of length width * height
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        set_panic_hook();
        web_sys::console::log_1(&"Creating a new Universe".into());

        let width = 64;
        let height = 64;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // construct the initial cells state
        for i in 0..size {
            // https://docs.rs/fixedbitset/0.2.0/fixedbitset/struct.FixedBitSet.html#method.set
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    fn construct_cells(&mut self) -> FixedBitSet {
        let size = (self.width * self.height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, i % 2 == 0 || i % 7 == 0);
        }

        cells
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.construct_cells();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.construct_cells();
    }

    /// Find the array index of the cell at a given row and column
    /// in the universe.
    /// Rows and columns start at 0.
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    /// The live_neighbor_count method uses deltas and modulo to avoid
    /// special casing the edges of the universe with `if`s.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // row and column can be 0, and if we attempted to
                // subtract 1 from them, there would be an unsigned
                // integer underflow.
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                // cells can be either 0 or 1, so here we just made
                // a simple sum an be sure that only the cells alive
                // will count
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);

                // cells can be accessed by using contains or using
                // the index syntax.
                // https://docs.rs/fixedbitset/0.2.0/fixedbitset/struct.FixedBitSet.html#method.contains
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                // To update a cell in the next tick of the universe,
                // we use the set method of FixedBitSet
                next.set(
                    idx,
                    match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise,
                    },
                );
            }
        }

        self.cells = next;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // To pass a pointer to the start of the bits to JavaScript,
    // you can convert the FixedBitSet to a slice and then
    // convert the slice to a pointer:
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

/// Public methods for testing (not exposed to javascript)
impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        let mut next = self.cells.clone();

        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            next.set(idx, true);
        }

        self.cells = next;
    }

    /// kill all the cells
    pub fn kill_all(&mut self) {
        let size = (self.width * self.height) as usize;
        let cells = FixedBitSet::with_capacity(size);

        self.cells = cells;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
