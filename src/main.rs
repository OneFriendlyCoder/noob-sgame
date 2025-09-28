// mod utilis;

// use macroquad::prelude::*;
// use utilis::{Movement, get_movement};
// #[macroquad::main("NOOB's GAME")]
// async fn main() {
//     // set_fullscreen(true);

//     let screen_h: f32 = screen_height();
//     let screen_w: f32 = screen_width();
//     let screen_d: f32 = screen_h.max(screen_w);
//     let road_half = 0.01;
//     let lane_half = 0.005;
//     let grass_half = 0.09;
//     let red_half = 0.32;

//     let mut camera = Camera3D {
//         position: vec3(0.0, screen_h*0.01, 0.0),
//         target: vec3(0.0, 0.0, screen_d),
//         up: vec3(0.0, 1.0, 0.0),
//         ..Default::default()
//     };

//     loop {
//         clear_background(BLUE);
//         set_camera(&camera);

//         draw_cube(
//             vec3(0.0, -1.0, 0.0),
//             vec3(screen_w * road_half * 2.0, 0.1, screen_d),
//             None,
//             DARKGRAY,
//         );

//         draw_cube(
//             vec3(-screen_w * (road_half + lane_half / 2.0), -1.0, 0.0),
//             vec3(screen_w * lane_half, 0.1, screen_d),
//             None,
//             BLACK,
//         );
//         draw_cube(
//             vec3(screen_w * (road_half + lane_half / 2.0), -1.0, 0.0),
//             vec3(screen_w * lane_half, 0.1, screen_d),
//             None,
//             BLACK,
//         );

//         draw_cube(
//             vec3(-screen_w * (road_half + lane_half + grass_half / 2.0), -1.0, 0.0),
//             vec3(screen_w * grass_half, 0.1, screen_d),
//             None,
//             GREEN,
//         );
//         draw_cube(
//             vec3(screen_w * (road_half + lane_half + grass_half / 2.0), -1.0, 0.0),
//             vec3(screen_w * grass_half, 0.1, screen_d),
//             None,
//             GREEN,
//         );

//         draw_cube(
//             vec3(-screen_w * (road_half + lane_half + grass_half + red_half / 2.0), -1.0, 0.0),
//             vec3(screen_w * red_half, 0.1, screen_d),
//             None,
//             RED,
//         );
//         draw_cube(
//             vec3(screen_w * (road_half + lane_half + grass_half + red_half / 2.0), -1.0, 0.0),
//             vec3(screen_w * red_half, 0.1, screen_d),
//             None,
//             RED,
//         );


//         //camera movement with keyboard
//         if let Some(m) = get_movement(){
//             match m{
//                 Movement::W => camera.position.z+=2.0,
//                 Movement::A => camera.position.x+=2.0,
//                 Movement::S => camera.position.z-=2.0,
//                 Movement::D => camera.position.x-=2.0,
//             }
//         }

//         //camera movement with mouse : the camera should always point towards the mouse
//         let pos = mouse_position();
//         camera.target = vec3(pos.0, 0.0, 0.0);
//         // println!("({},{})", pos.0, pos.1);

//         next_frame().await;
//     }
// }



mod utilis;

use macroquad::prelude::*;
use utilis::{Movement, get_movement};

struct Player {
    position: Vec3,
    yaw: f32,   // rotation around Y-axis
    pitch: f32, // rotation around X-axis
}

#[macroquad::main("NOOB's GAME")]
async fn main() {
    let screen_h: f32 = screen_height();
    let screen_w: f32 = screen_width();
    let screen_d: f32 = screen_h.max(screen_w);
    let road_half = 0.01;
    let lane_half = 0.005;
    let grass_half = 0.09;
    let red_half = 0.32;

    // initialize player
    let mut player = Player {
        position: vec3(0.0, screen_h * 0.01, 0.0),
        yaw: 0.0,
        pitch: 0.0,
    };

    let mut camera = Camera3D {
        position: player.position,
        target: vec3(0.0, 0.0, screen_d),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    // hide cursor for FPS feel
    show_mouse(false);

    loop {
        clear_background(BLUE);

        // --- Mouse look ---
        let mouse_delta = mouse_position();
        let sensitivity = 0.2;

        player.yaw += mouse_delta.0 * sensitivity;
        player.pitch -= mouse_delta.1 * sensitivity;
        player.pitch = player.pitch.clamp(-89.0, 89.0);

        // --- Calculate forward and right vectors ---
        let yaw_rad = player.yaw.to_radians();
        let pitch_rad = player.pitch.to_radians();

        let forward = vec3(
            yaw_rad.sin() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.cos() * pitch_rad.cos(),
        )
        .normalize();

        let right = forward.cross(vec3(0.0, 1.0, 0.0)).normalize();

        // --- Keyboard movement ---
        let speed = 2.0;
        if is_key_down(KeyCode::W) {
            player.position += forward * speed;
        }
        if is_key_down(KeyCode::S) {
            player.position -= forward * speed;
        }
        if is_key_down(KeyCode::A) {
            player.position -= right * speed;
        }
        if is_key_down(KeyCode::D) {
            player.position += right * speed;
        }

        // --- Update camera ---
        camera.position = player.position;
        camera.target = player.position + forward;
        camera.up = vec3(0.0, 1.0, 0.0);

        set_camera(&camera);

        // --- Draw road + lanes + grass + red zones ---
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

        next_frame().await;
    }
}
