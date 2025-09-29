mod utilis;

use macroquad::prelude::*;
use utilis::{Movement, get_movement};

#[macroquad::main("RUSTY KRUNKER")]
async fn main() {
    set_pc_assets_folder("./assets/textures/");
    let screen_h: f32 = screen_height();
    let screen_w: f32 = screen_width();
    let screen_d: f32 = screen_h.max(screen_w);

    let road_half = 0.01;
    let lane_half = 0.005;
    let grass_half = 0.09;
    let red_half = 0.32;

    const MOUSE_SENSITIVITY: f32 = 0.005;

    let mut camera = Camera3D {
        position: vec3(0.0, screen_h * 0.001, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };
    let texture: Texture2D = load_texture("crosshair.png").await.unwrap();

    loop {
        clear_background(BLUE);
        set_camera(&camera);

        // starting 3D drawing
        draw_cube(
            vec3(0.0, -1.0, 0.0),
            vec3(screen_w * road_half * 2.0, 0.1, screen_d),
            None,
            DARKGRAY,
        );

        draw_cube(
            vec3(-screen_w * (road_half + lane_half / 2.0), -1.0, 0.0),
            vec3(screen_w * lane_half, 0.1, screen_d),
            None,
            BLACK,
        );
        draw_cube(
            vec3(screen_w * (road_half + lane_half / 2.0), -1.0, 0.0),
            vec3(screen_w * lane_half, 0.1, screen_d),
            None,
            BLACK,
        );

        draw_cube(
            vec3(-screen_w * (road_half + lane_half + grass_half / 2.0), -1.0, 0.0),
            vec3(screen_w * grass_half, 0.1, screen_d),
            None,
            GREEN,
        );
        draw_cube(
            vec3(screen_w * (road_half + lane_half + grass_half / 2.0), -1.0, 0.0),
            vec3(screen_w * grass_half, 0.1, screen_d),
            None,
            GREEN,
        );

        draw_cube(
            vec3(-screen_w * (road_half + lane_half + grass_half + red_half / 2.0), -1.0, 0.0),
            vec3(screen_w * red_half, 0.1, screen_d),
            None,
            RED,
        );
        draw_cube(
            vec3(screen_w * (road_half + lane_half + grass_half + red_half / 2.0), -1.0, 0.0),
            vec3(screen_w * red_half, 0.1, screen_d),
            None,
            RED,
        );

        let screen_center = vec2(screen_w / 2.0, screen_h / 2.0); // screen_center : is the mid point of the gaming window
        let mouse_pos_tuple = mouse_position();
        let mouse_pos = vec2(mouse_pos_tuple.0, mouse_pos_tuple.1);
        let offset = mouse_pos - screen_center;

        let yaw = -(offset.x) * MOUSE_SENSITIVITY;
        let forward = vec3(yaw.cos(), 0.0, yaw.sin());
        if let Some(m) = get_movement() {
            match m {
                Movement::W => camera.position += forward,
                Movement::S => camera.position -= forward,
            }
        }
        camera.target = camera.position + forward;


        // starting 2D drawing
        set_default_camera();   // necessary for drawing 2D UI on the scree, switches drawing context to 2D, all coordinates are screen-pixels
        
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

        let text = "RUSTY KRUNKER";
        let font_size = 100.0;
        let color = RED;
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
        let text_width = text_dimensions.width;

        let x = (screen_width() - text_width) / 2.0;
        let y = font_size + 10.0;

        draw_text(text, x, y, font_size, color);

        next_frame().await;
    }
}