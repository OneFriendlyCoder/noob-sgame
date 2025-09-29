use macroquad::prelude::*;

#[derive(Debug)]
pub struct Enemy{
    texture: Texture2D,
    position: Vec3,
    size: Vec2,
    health: i32,
}

impl Enemy{             
    async fn new(texture_path: &str, position: Vec3, size: Vec2) -> Self {            //constructor function
        let texture = load_texture(texture_path).await.unwrap();
        Self {
            texture,
            position,
            size,
            health : 100,
        }
    }

    fn draw(&self){
        draw_texture_ex(
            &self.texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams{
                dest_size: Some(self.size),
                ..Default::default()
            },
        );
    }
}

#[derive(Debug)]
pub struct Enemies {
    enemies: Vec<Enemy>,
    size: i32,
}

impl Enemies{
    // init enemies
    pub async fn init_enemies(size: i32) -> Self {
        let mut enemies: Vec<Enemy> = Vec::new();
        for i in 0..size{
            let enemy = Enemy::new(
                "enemy/type1.png",
                vec3(50.0 * i as f32, 0.0, 100.0),
                vec2(10.0, 10.0),
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
