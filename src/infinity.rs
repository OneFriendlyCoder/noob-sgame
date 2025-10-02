// Draw the stars

use macroquad::prelude::*;

pub struct Star {
    pub pos: Vec3,
    pub size: f32,
    pub color: Color,
}

pub struct StarWall {
    pub stars: Vec<Star>,
    pub side: usize,
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
    pub padding: f32,
}

impl StarWall {
    pub fn new(
        count: usize,
        side: usize,
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        z_min: f32,
        z_max: f32,
        padding: f32,
    ) -> Self {
        let mut stars = Vec::new();
        let colors = [WHITE, YELLOW, GRAY, LIGHTGRAY, GOLD, SKYBLUE];

        for _ in 0..count {
            let pos = match side {
                0 => vec3(
                    x_max + padding,
                    rand::gen_range(y_min, y_max),
                    rand::gen_range(z_min, z_max),
                ),
                1 => vec3(
                    x_min - padding,
                    rand::gen_range(y_min, y_max),
                    rand::gen_range(z_min, z_max),
                ),
                2 => vec3(
                    rand::gen_range(x_min, x_max),
                    y_max + padding,
                    rand::gen_range(z_min, z_max),
                ),
                3 => vec3(
                    rand::gen_range(x_min, x_max),
                    y_min - padding,
                    rand::gen_range(z_min, z_max),
                ),
                4 => vec3(
                    rand::gen_range(x_min, x_max),
                    rand::gen_range(y_min, y_max),
                    z_max + padding,
                ),
                5 => vec3(
                    rand::gen_range(x_min, x_max),
                    rand::gen_range(y_min, y_max),
                    z_min - padding,
                ),
                _ => vec3(0.0, 0.0, 0.0),
            };

            stars.push(Star {
                pos,
                size: rand::gen_range(0.05, 0.5),
                color: colors[rand::gen_range(0, colors.len())],
            });
        }

        Self {
            stars,
            side,
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            padding,
        }
    }

    pub fn draw(&self) {
        for star in &self.stars {
            draw_cube(star.pos, vec3(star.size, star.size, star.size), None, star.color);
        }
    }
}
