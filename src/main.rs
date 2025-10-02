mod utilis;
mod enemy;
mod collision;
mod player;
mod grid;

use macroquad::prelude::*;
use utilis::{Movement, get_movement};
use enemy::*;
use collision::*;
use player::*;
use grid::*;

#[macroquad::main("RUSTY KRUNKER")]
async fn main() {
    set_pc_assets_folder("./assets/");
    let screen_h: f32 = screen_height();
    let screen_w: f32 = screen_width();
    let screen_d: f32 = screen_h.max(screen_w);
    let road_half = 0.01;
    let lane_half = 0.005;
    let grass_half = 0.09;
    let red_half = 0.32;
    let x_min = -screen_w * (road_half + lane_half + grass_half + red_half);
    let x_max =  screen_w * (road_half + lane_half + grass_half + red_half);
    let z_min = -screen_d / 2.0;
    let z_max =  screen_d / 2.0;

    const MOUSE_SENSITIVITY: f32 = 0.005;
    let mut player: Player = Player::new(vec3(0.0, screen_h * 0.001, 0.0),vec3(0.0, 0.0, 0.0) ,"Player1".to_string(), "Shotgun".to_string());
    
    let mut camera = Camera3D {
        position: player.position,
        target: player.target,
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };
    
    let texture: Texture2D = load_texture("textures/crosshair.png").await.unwrap();
    let enemies = Enemies::init_enemies(3, x_min, x_max, z_min, z_max).await;
    let grid = init_grid(&enemies, x_min, x_max, z_min, z_max, 10, 10);
    
    loop {
        clear_background(BLACK);
        set_camera(&camera);
        draw_cube(
            vec3(0.0, -1.0, 0.0),
            vec3(
                screen_w * (road_half*2.0 + 2.0*lane_half + 2.0*grass_half + 2.0*red_half), 
                0.1, 
                screen_d
            ),
            None,
            DARKGRAY,
        );

        let screen_center = vec2(screen_w / 2.0, screen_h / 2.0); // screen_center : is the mid point of the gaming window
        let mouse_pos_tuple = mouse_position();
        let mouse_pos = vec2(mouse_pos_tuple.0, mouse_pos_tuple.1);
        let offset = mouse_pos - screen_center;

        // yaw, pitch and strafing done
        let yaw = -(offset.x) * MOUSE_SENSITIVITY;
        let mut pitch = -(offset.y) * MOUSE_SENSITIVITY;
        let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.1;
        pitch = pitch.clamp(-pitch_limit, pitch_limit);
        let look = vec3(yaw.cos()*pitch.cos(), pitch.sin(), yaw.sin()*pitch.cos());
        let forward = vec3(yaw.cos(), 0.0, yaw.sin());
        let strafe_dir = vec3(-forward.z, 0.0, forward.x);
        // let previous_player_position = player.position;
        player.update_player_position(forward, strafe_dir, look, &enemies, &grid, &mut camera);
        // camera.position = player.position;
        // camera.target = player.target;
        
        // if let Some(m) = get_movement() {
        //     match m {
        //         Movement::W => camera.position += forward,
        //         Movement::S => camera.position -= forward,
        //         Movement::A => camera.position -= strafe_dir,
        //         Movement::D => camera.position += strafe_dir,
        //     }
        // }

        // player.target = player.position + look;

        // collision detection
        // if detect_collision(&enemies, &player){
        //     player.position = previous_player_position;
        //     player.target = player.position + look;
        // }


        enemies.draw_enemies();
        set_default_camera();   // necessary for drawing 2D UI on the screen, switches drawing context to 2D, all coordinates are screen-pixels
        
        //crosshair 
        let size:Vec2 = vec2(texture.width()/20.0, texture.height()/20.0);
        let x = (screen_width() - size.x) / 2.0;
        let y = (screen_height() - size.y) / 2.0;
        draw_texture_ex(
            &texture,
            x,
            y,
            RED,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
        

        next_frame().await;
    }
}