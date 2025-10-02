use macroquad::prelude::*;
use crate::utilis::*;
use crate::enemy::*;
use crate::collision::*;
use crate::grid::*;

pub struct Player {
    pub health: u32,
    pub name: String,
    pub weapon: String,
    pub position: Vec3,
    pub target: Vec3,
    pub bullets: u32,
    pub targets_shot : u32,
    pub speed: f32,
}

impl Player{
    pub fn new(position: Vec3, target: Vec3 ,name: String, weapon: String) -> Self{
        Self {
            health: 100,
            name,
            weapon,
            position,
            target, 
            bullets: 0,
            targets_shot: 0,
            speed: 2.0,
        }
    }
    pub fn update_player_position(&mut self, forward: Vec3, strafe_dir: Vec3, look: Vec3, enemies: &Enemies, grid: &Grid, camera: &mut Camera3D) {
        let previous_position = self.position;
        if let Some(m) = get_movement() {
            match m {
                Movement::W => self.position += forward * self.speed,
                Movement::S => self.position -= forward * self.speed,
                Movement::A => self.position -= strafe_dir * self.speed,
                Movement::D => self.position += strafe_dir * self.speed,
            }
        }
        if detect_collision(enemies, grid, self) {
            println!("Collision detected");
            self.position = previous_position;
        }
        self.target = self.position + look;
        camera.position = self.position;
        camera.target = self.target;
    }

}