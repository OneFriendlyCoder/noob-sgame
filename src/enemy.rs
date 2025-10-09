use macroquad::prelude::*;
use crate::grid::*;

#[derive(Debug)]
pub struct Enemy {
    pub uid: u32,
    pub position: Vec3,
    pub size: Vec3,
    pub health: i32,
    pub ttl: f32,
    pub weight: u32,
    pub color: Color, // add this
}

impl Enemy {
    pub async fn new(uid: u32, position: Vec3, size: Vec3, ttl: f32, weight: u32, color:Color) -> Self {    
        Self {
            uid,
            position,
            size,
            health: 100,
            ttl,
            weight,
            color,
        }
    }

        pub fn draw(&self) {
            draw_cube(self.position, self.size, None, self.color);
        }


    pub fn update_ttl(&mut self, delta_time: f32) {
        self.ttl -= delta_time;
    }

    pub fn is_dead(&self) -> bool {
        self.ttl <= 0.0 || self.health <= 0
    }
}

#[derive(Debug)]
pub struct Enemies {
    pub enemies: Vec<Enemy>,
}

impl Enemies {
    pub async fn init_enemies(size: i32, xmin: f32, xmax: f32, zmin: f32, zmax: f32) -> Self {
        let mut enemies: Vec<Enemy> = Vec::new();
        let y_measure = 20.0;
        let ground_y = -1.0;

        let colors = [GREEN, SKYBLUE, PINK, YELLOW, WHITE, RED];
        for i in 0..size {
            let x = rand::gen_range(xmin, xmax);
            let z = rand::gen_range(zmin, zmax);
            let ttl = rand::gen_range(10.0, 30.0);
            let color_idx = (i as usize) % colors.len();
            let weight = match color_idx {
                0 => 1,
                1 => 2,
                2 => 3,
                3 => 4,
                4 => 5,
                5 => 6,
                _ => 1,
            };

            let enemy = Enemy::new(
                i as u32,
                vec3(x, ground_y + y_measure / 2.0, z),
                vec3(5.0, y_measure, 5.0),
                ttl,
                weight,
                colors[color_idx],
            ).await;

            enemies.push(enemy);
        }

        Self { enemies }
    }


    pub fn draw_enemies(&self) {
        for enemy in &self.enemies {
            enemy.draw();
        }
    }

    pub fn update_enemies(&mut self, delta_time: f32) {
        self.enemies.iter_mut().for_each(|e| e.update_ttl(delta_time));
        self.enemies.retain(|e| !e.is_dead());
    }

    pub fn update_enemies_grid(&mut self, delta_time: f32, grid: &mut Grid) {
        self.enemies.iter_mut().for_each(|e| e.update_ttl(delta_time));

        let mut i = 0;
        while i < self.enemies.len() {
            if self.enemies[i].is_dead() {
                let last_idx = self.enemies.len() - 1;
                let enemy_pos = self.enemies[i].position;
                let enemy_size = self.enemies[i].size;
                let (x0, z0) = grid.get_cell_coords(enemy_pos - enemy_size / 2.0);
                let (x1, z1) = grid.get_cell_coords(enemy_pos + enemy_size / 2.0);

                for x in x0..=x1 {
                    for z in z0..=z1 {
                        grid.cells[x][z].retain(|&idx| idx != i);
                    }
                }
                self.enemies.swap_remove(i);
                if i != last_idx {
                    let moved_enemy = &self.enemies[i];
                    let (mx0, mz0) = grid.get_cell_coords(moved_enemy.position - moved_enemy.size / 2.0);
                    let (mx1, mz1) = grid.get_cell_coords(moved_enemy.position + moved_enemy.size / 2.0);

                    for x in mx0..=mx1 {
                        for z in mz0..=mz1 {
                            for idx in &mut grid.cells[x][z] {
                                if *idx == last_idx {
                                    *idx = i;
                                }
                            }
                        }
                    }
                }
            } else {
                i += 1;
            }
        }
    }
}
