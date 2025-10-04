use macroquad::prelude::*;
use crate::utilis::*;
use crate::enemy::*;
use crate::collision::*;
use crate::grid::*;
use crate::camera::*;
// use macroquad::prelude::MouseButton::Right;

pub struct Shot{
    pub start: Vec3,
    pub end: Vec3,
    pub lifetime: f32,
}

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
    pub shots: Vec<Shot>,
    // pub camera1_yaw: f32,
    // pub camera1_pitch: f32,
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
            shots: vec![],
            // camera1_yaw: 0.0,
            // camera1_pitch: 0.0,
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

    pub fn update_player_position(&mut self, forward: Vec3, strafe_dir: Vec3, look: Vec3, enemies: &Enemies, grid: &Grid, camera: &mut Camera3D, camera1: &mut Camera3D,camera_view: CameraView) {
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

        match camera_view {
            CameraView::FirstPerson => {
                camera.position = self.position;
                camera.target = self.position + look;
            }
            CameraView::ThirdPerson => {
                let camera_offset = vec3(0.0, 10.0, -10.0);
                let rotated_offset = vec3(
                    camera_offset.x * self.yaw.cos() - camera_offset.z * self.yaw.sin(),
                    camera_offset.y,
                    camera_offset.x * self.yaw.sin() + camera_offset.z * self.yaw.cos(),
                );
                camera1.position = self.position + rotated_offset;
                camera1.target = self.position + look;
                                    
                }
            }


        // changing fov, scope effect
        let target_fovy = if is_mouse_button_down(MouseButton::Right) {
            //shooting logic
            let o = camera.position;
            let d = (camera.target - camera.position).normalize();
            let md = 100000.0;
            let ep = o + d*md;
            
            if is_mouse_button_down(MouseButton::Left){
                self.shots.push(Shot{
                    start: o,
                    end: ep,
                    lifetime: 200.0,
                });

            }
            
            45.0_f32.to_radians()
        } else {
            60.0_f32.to_radians()
        };
        camera.fovy += (target_fovy - camera.fovy) * 0.2;

        for shot in &mut self.shots{
            draw_line_3d(shot.start, shot.end, BLUE);
            shot.lifetime -= get_frame_time();
        }
        self.shots.retain(|s| s.lifetime>0.0);      //deletes all other shots whose lifetime is less than 0


    }

}