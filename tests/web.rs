//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate game_of_life_wasm_haochun;
use wasm_bindgen_test::*;
use game_of_life_wasm_haochun::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn input_pulsar() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(13);
    universe.set_height(13);
 
    for r in [0, 5, 7, 12].iter().cloned() {
        universe.set_cells(&[(r,2), (r,3), (r,4), (r,8), (r,9), (r,10)]);
        universe.set_cells(&[(2,r), (3,r), (4,r), (8,r), (9,r), (10,r)]);
    }
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
 
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

#[cfg(test)]
pub fn empty_universe(width: u32, height: u32) -> Universe {
    let mut universe = Universe::new();
    universe.set_width(width);
    universe.set_height(height);
    universe
}

#[wasm_bindgen_test]
pub fn test_reset() {
    let mut universe = input_spaceship();
    universe.reset(1.0);

    let empty = empty_universe(6, 6);
    assert_eq!(universe.get_cells(), empty.get_cells());
}

#[wasm_bindgen_test]
pub fn test_purge() {
    let mut universe = input_spaceship();
    universe.purge();

    let empty = empty_universe(6, 6);
    assert_eq!(universe.get_cells(), empty.get_cells());
}

#[wasm_bindgen_test]
pub fn test_toggle_cell() {
    let mut input_universe = input_spaceship();
    input_universe.toggle_cell(2, 2);

    assert!(input_universe.get_cells()[14 as usize]);
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();
    let expected_universe = expected_spaceship();

    input_universe.tick();
    assert_eq!(input_universe.get_cells(), expected_universe.get_cells());
}

#[wasm_bindgen_test]
pub fn test_add_glider() {
    let mut universe = empty_universe(6, 6);
    let spaceship = input_spaceship();

    universe.add_glider(2, 2);
    assert_eq!(universe.get_cells(), spaceship.get_cells());
}

#[wasm_bindgen_test]
pub fn test_add_pulsar() {
    let mut universe = empty_universe(13, 13);
    let pulsar = input_pulsar();

    universe.add_pulsar(6, 6);
    assert_eq!(universe.get_cells(), pulsar.get_cells());
}

