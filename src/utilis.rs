use macroquad::prelude::*;

pub enum Movement {
    W,
    // A,
    S,
    // D,
}

pub fn get_movement() -> Option<Movement> {
    if is_key_down(KeyCode::W) {
        Some(Movement::W)
    // } else if is_key_down(KeyCode::A) {
    //     Some(Movement::A)
    } else if is_key_down(KeyCode::S) {
        Some(Movement::S)
    // } else if is_key_down(KeyCode::D) {
    //     Some(Movement::D)
    } else {
        None
    }
}
