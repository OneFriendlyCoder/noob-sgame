use macroquad::prelude::*;
use crate::utilis::*;
use crate::enemy::*;
use crate::collision::*;
use crate::grid::*;
use crate::camera::*;
// use macroquad::prelude::MouseButton::Right;

pub struct Player {
    pub health: u32,
    pub name: String,
    pub weapon: String,
    pub position: Vec3,
    pub target: Vec3,           // the point at which the player is looking at
    pub bullets: u32,
    pub targets_shot : u32,
    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub velocity_y: f32,
    pub is_jumping: bool,
    pub size: Vec3,
}

impl Player{
    pub fn new(position: Vec3, target: Vec3 ,name: String, weapon: String, yaw:f32, pitch:f32) -> Self{
        Self {
            health: 100,
            name,
            weapon,
            position,
            target, 
            bullets: 0,
            targets_shot: 0,
            speed: 1.0,
            yaw, 
            pitch,
            velocity_y: 0.0,
            is_jumping: false,
            size: vec3(1.0,1.0,1.0),
        }
    }

        pub fn draw_player(&self, camera_view: &CameraView) {
            if let CameraView::ThirdPerson = camera_view {
                draw_cube(
                    self.position,
                    self.size,
                    None,
                    RED,
                );
            }
        }

    pub fn update_player_position(&mut self, forward: Vec3, strafe_dir: Vec3, look: Vec3, enemies: &Enemies, grid: &Grid, camera: &mut Camera3D, camera_view: CameraView) {
        let previous_position = self.position;
        let movements = get_movement();

        for m in movements {
            match m {
                Movement::W => self.position += forward * self.speed,
                Movement::S => self.position -= forward * self.speed,
                Movement::A => self.position -= strafe_dir * self.speed,
                Movement::D => self.position += strafe_dir * self.speed,
            }
        }

        // jump mechanics
        if is_key_pressed(KeyCode::Space) && !self.is_jumping{
            self.velocity_y = 2.0;
            self.is_jumping = true;
        }

        if self.is_jumping{
            self.position.y += self.velocity_y;
            self.velocity_y -= 0.2;
            if self.position.y <= 0.0 {
                self.position.y = 0.0;
                self.is_jumping = false;
                self.velocity_y = 0.0;
            }
        }

        if detect_collision(enemies, grid, self) {
            println!("Collision detected");
            self.position = previous_position;
        }

        match camera_view{
            CameraView::FirstPerson => {
                camera.position = self.position;
                camera.target = self.position + look;
            }
            CameraView::ThirdPerson => {
                let camera_offset = vec3(0.0, 2.0, -5.0);
                camera.position = self.position + camera_offset;
                camera.target = self.position; 
            }
        }

        // changing fov, scope effect
        let target_fovy = if is_mouse_button_down(MouseButton::Right) {
            45.0_f32.to_radians()
        } else {
            60.0_f32.to_radians()
        };
        camera.fovy += (target_fovy - camera.fovy) * 0.2;


    }

}