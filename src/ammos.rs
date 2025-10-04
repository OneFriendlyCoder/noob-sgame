use macroquad::prelude::*;

pub struct Bullet {
    pub position: Vec3,
    pub velocity: Vec3,
    pub speed: f32,
}

impl Bullet {
    pub fn update(&mut self) {
        self.position += self.velocity * self.speed * get_frame_time();
    }

    pub fn draw(&self) {
        draw_sphere(self.position, 0.1, YELLOW);
    }
}
