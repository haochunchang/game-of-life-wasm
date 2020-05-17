mod utils;

extern crate fixedbitset;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use js_sys::Math::random;
use fixedbitset::FixedBitSet;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    next: FixedBitSet,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {

    /// Initialize a new Universe with random 50% Alive cells.
    ///
    /// Defaults to 80 x 80 px Universe Grid.
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 80;
        let height = 80;
        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        for i in 0..size {
            cells.set(i, random() > 0.5);
        }
		let next = cells.clone();

        Universe {
            width,
            height,
            cells,
			next,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the height of the Universe.
    ///
    /// Reset all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = (self.width * height) as usize;

        self.cells = FixedBitSet::with_capacity(size);
        self.next = self.cells.clone();
        self.purge();
    }

    /// Set the width of the Universe.
    ///
    /// Reset all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = (width * self.height) as usize;

        self.cells = FixedBitSet::with_capacity(size);
        self.next = self.cells.clone();
        self.purge();
    }

    /// Return a slice pointer to cells.
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Toggle cells by changing their states.
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells.toggle(idx);
    }

    /// Set all cells to dead cells.
    pub fn purge(&mut self) {
        self.cells.set_range(.., false);
        self.next.set_range(.., false);
    }

    /// Randomly set cells to alive or dead by given dead probability.
    pub fn reset(&mut self, dead_proba: f64) {
        for i in 0..self.width * self.height {
            self.cells.set(i as usize, random() > dead_proba);
        }
    }

    /// Simulate the next generation of the Universe.
    pub fn tick(&mut self) {

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);	
                let live_neighbors = self.live_neighbor_count(row, col);

                self.next.set(idx, match (self.cells[idx], live_neighbors) { 
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true, 
                    (otherwise, _) => otherwise,
                });
            }
        }
        self.cells = self.next.clone()
    }

    /// Add the pulsar pattern centered at the given row/column.
    ///
    /// Reference: https://www.conwaylife.com/w/images/4/49/Pulsar.png
    pub fn add_pulsar(&mut self, row: u32, column: u32) {

        for delta_col in [-6, -1, 1, 6].iter().cloned() {
            for delta_row in [-4, -3, -2, 2, 3, 4].iter().cloned() {
                let mut new_row = self.wrap_row(row, delta_row);
                let mut new_col = self.wrap_column(column, delta_col);
                let mut idx = self.get_index(new_row, new_col); 
                self.cells.set(idx, true);

                new_row = self.wrap_row(row, delta_col);
                new_col = self.wrap_column(column, delta_row);
                idx = self.get_index(new_row, new_col);
                self.cells.set(idx, true);
            }
        }
    }

    /// Add a glider (direction: buttom-right) 
    pub fn add_glider(&mut self, row: u32, column: u32) {
        let deltas = [(-1,0), (1,0), (0,1), (1,-1), (1,1)];

        for (delta_row, delta_col) in deltas.iter().cloned() {
            let r = self.wrap_row(row, delta_row);
            let c = self.wrap_column(column, delta_col);
            let idx = self.get_index(r, c);
            self.cells.set(idx, true);
        }
    }
}


impl Universe {

    /// Get the dead and alive values of the entire Universe.
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }

    /// Get cell's index from WebAssembly linear memory.
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn wrap_row(&self, row: u32, delta: i32) -> u32 {
        let row = row as i32;
        let new_row = row + delta;
        let result = match new_row {
            x if x < 0 => x as u32 + self.height,
            x if x > (self.height as i32 - 1) => x as u32 - self.height,
            x => x as u32,
        };
        result
    }

    fn wrap_column(&self, column: u32, delta: i32) -> u32 {
        let col = column as i32;
        let new_col = col + delta;
        let result = match new_col {
            x if x < 0 => x as u32 + self.width,
            x if x > (self.width as i32 - 1) => x as u32 - self.width,
            x => x as u32,
        };
        result
    }    

    /// Given a single cell, count the number of alive neighbors.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

       	let north = if row == 0 { self.height - 1 } else { row - 1 }; 
       	let south = if row == self.height - 1 { 0 } else { row + 1 }; 
       	let west = if column == 0 { self.width - 1 } else { column - 1 }; 
       	let east = if column == self.width - 1 { 0 } else { column + 1 }; 

		let ns = [north, south, row];
		let we = [west, east, column]; 
		for (i, r) in ns.iter().cloned().enumerate() {
			for (j, c) in we.iter().cloned().enumerate() {
                if i == 2 && j == 2 {
                    break;
                }
				let idx = self.get_index(r, c);
				count += self.cells[idx] as u8;
			}
		}
        count
    }
}
