use macroquad::prelude::*;

pub struct Player {
    health: u32,
    name: String,
    weapon: String,
    position: f32,
    bullets: u32,
    targets_shot : u32,
}