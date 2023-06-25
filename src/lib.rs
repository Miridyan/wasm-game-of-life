mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq)]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self  {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|cell| if cell % 2 == 0 || cell % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            })
            .collect();

        Self { width, height, cells }
    }

    pub fn index(&self, x: u32, y: u32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn live_neighbor_count(&self, x: u32, y: u32) -> u32 {
        (x.checked_sub(1).unwrap_or(0)..=x.checked_add(1).unwrap_or(u32::MAX).min(self.width-1))
            .map(|xp| (y.checked_sub(1).unwrap_or(0)..=y.checked_add(1).unwrap_or(u32::MAX).min(self.height-1))
                .map(move |yp| (xp, yp)))
            .flatten()
            .filter(|(xp, yp)| *xp != x || *yp != y)
            .fold(0, |acc, (xp, yp)| acc + self.cells[self.index(xp, yp)] as u32)
    }

    pub fn tick(&mut self) {
        self.cells = self.cells.iter()
            .enumerate()
            .map(|(idx, cell)| {
                let x = idx as u32 % self.width;
                let y = idx as u32 / self.width;

                match (cell, self.live_neighbor_count(x, y)) {
                    (&Cell::Alive, 0..=1) | (&Cell::Alive, 4..) => Cell::Dead,
                    (&Cell::Alive, 2..=3) | (&Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => *otherwise
                }
            })
            .collect::<Vec<Cell>>();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for cell in line {
                write!(f, "{}", if cell == &Cell::Dead { '◻' } else { '◼' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("This is a game of life");
}
