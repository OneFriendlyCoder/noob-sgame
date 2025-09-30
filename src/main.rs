mod utilis;
mod enemy;

use macroquad::prelude::*;
use utilis::{Movement, get_movement};
use enemy::*;

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

    let mut camera = Camera3D {
        position: vec3(0.0, screen_h * 0.001, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };
    let texture: Texture2D = load_texture("textures/crosshair.png").await.unwrap();
    let enemies = Enemies::init_enemies(100, x_min, x_max, z_min, z_max).await;

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
        if let Some(m) = get_movement() {
            match m {
                Movement::W => camera.position += forward,
                Movement::S => camera.position -= forward,
                Movement::A => camera.position -= strafe_dir,
                Movement::D => camera.position += strafe_dir,
            }
        }
        camera.target = camera.position + look;


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

        // // GAME NAME
        // let text = "RUSTY KRUNKER";
        // let font_size = 100.0;
        // let color = RED;
        // let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        // let text_width = text_dimensions.width;

        // let x = (screen_width() - text_width) / 2.0;
        // let y = font_size + 10.0;

        // draw_text(text, x, y, font_size, color);

        next_frame().await;
    }
}