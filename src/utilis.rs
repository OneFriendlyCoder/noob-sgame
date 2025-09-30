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

pub fn get_movement() -> Option<Movement> {
    if is_key_down(KeyCode::W) {
        Some(Movement::W)
    } else if is_key_down(KeyCode::A) {
        Some(Movement::A)
    } else if is_key_down(KeyCode::S) {
        Some(Movement::S)
    } else if is_key_down(KeyCode::D) {
        Some(Movement::D)
    } else {
        None
    }
}




// obj parsing to create mesh
// function will take the .obj file as an parameter
// and return a vector of meshes that are parsed from the obj file, each mesh each object

// pub fn create_mesh(filepath: &str) -> Vec<Mesh>{
//     let contents = fs::read_to_string(filepath).expect("Error reading file");
    
// }