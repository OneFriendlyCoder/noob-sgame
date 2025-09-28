mod utilis;

use macroquad::prelude::*;
use utilis::{Movement, get_movement};
#[macroquad::main("NOOB's GAME")]
async fn main() {
    // set_fullscreen(true);

    let screen_h: f32 = screen_height();
    let screen_w: f32 = screen_width();
    let screen_d: f32 = screen_h.max(screen_w);
    let road_half = 0.01;
    let lane_half = 0.005;
    let grass_half = 0.09;
    let red_half = 0.32;

    let mut camera = Camera3D {
        position: vec3(0.0, screen_h*0.1, 0.0),
        target: vec3(0.0, 0.0, screen_d),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(BLUE);
        set_camera(&camera);

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

        // mouse movement
        // let (mx, my) = mouse_position();
        // let ndc_x = (mx / screen_w) * 2.0 - 1.0;
        // let ndc_y = 1.0 - (my / screen_h) * 2.0;
        // let world_x = ndc_x * 10.0;
        // let world_y = if 0.0 < (ndc_y * 10.0) {ndc_y*10.0} else {0.0};
        
        let (mx, my) = mouse_position();
        let world_x = mx / screen_w * screen_w as f32 * 2.0 - screen_w as f32;
        let world_y = screen_h as f32 - my / screen_h * screen_h as f32 * 2.0;
        camera.target = vec3(world_x, world_y,0.0 );

    

        //camera movement with keyboard
        if let Some(m) = get_movement(){
            let mut forward = camera.target - camera.position;
            forward.y = 0.0;
            let forward = forward.normalize();
            
            // let mut right = forward.cross(camera.up);
            // right.y = 0.0;
            // let right = right.normalize();

            match m{
                Movement::W => {
                    camera.position += forward;
                    camera.target += forward;  
                },
                // Movement::A => {
                //     camera.position += right;
                //     camera.target += right;                      
                // },
                Movement::S => {
                    camera.position -= forward;
                    camera.target -= forward;  
                },
                // Movement::D => {
                //     camera.position -= right;
                //     camera.target -= right;  
                // },
            }
        }



        next_frame().await;
    }
}