use macroquad::prelude::*;
use macroquad::models::*;
use std::env;
use std::fs;

pub enum Movement {
    W,
    A,
    S,
    D,
}

pub fn get_movement() -> Vec<Movement> {
    let mut movements: Vec<Movement> = Vec::new();
    if is_key_down(KeyCode::W) {
        movements.push(Movement::W);
    }
    if is_key_down(KeyCode::A) {
        movements.push(Movement::A);
    }
    if is_key_down(KeyCode::S) {
        movements.push(Movement::S);
    }
    if is_key_down(KeyCode::D) {
        movements.push(Movement::D);
    }
    movements
}


