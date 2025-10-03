use macroquad::prelude::*;

#[derive(Debug)]
pub struct Enemy{
    // texture: Texture2D,
    pub position: Vec3,             //center
    pub size: Vec3,
    health: i32,
}

impl Enemy{             
    // async fn new(texture_path: &str, position: Vec3, size: Vec3) -> Self {            //constructor function
    async fn new(position: Vec3, size: Vec3) -> Self {    
        // let texture = load_texture(texture_path).await.unwrap();
        Self {
            // texture,
            position,
            size,
            health : 100,
        }
    }

    fn draw(&self){
        draw_cube(
            self.position,
            self.size,
            None,
            WHITE,
        )
    }
}

#[derive(Debug)]
pub struct Enemies {
    pub enemies: Vec<Enemy>,
    pub size: i32,
}

impl Enemies{
    // init enemies
    pub async fn init_enemies(size: i32, xmin:f32, xmax:f32, zmin:f32, zmax:f32) -> Self {
        let mut enemies: Vec<Enemy> = Vec::new();
        let y_measure = 20.0;
        for _ in 0..size{
            let x = rand::gen_range(xmin,xmax);
            let z = rand::gen_range(zmin,zmax);
            let enemy = Enemy::new(
                // "enemy/type1.png",
                vec3(x, y_measure/2.0, z),
                vec3(5.0, y_measure, 5.0),
            ).await;
            enemies.push(enemy);
        }

        Self {enemies, size}
    }
    // draw enemies
    pub fn draw_enemies(&self){
        for enemy in &self.enemies {
            enemy.draw();
        }
    }
}

