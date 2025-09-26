// use macroquad::prelude::*;

// #[macroquad::main("BasicShapes")]
// async fn main() {
//     // let mut color_c:Color = RED;
//     loop {
//         clear_background(BLACK);
    
//         // let x = 1000.0;
//         // let y = 700.0;
//         // let text = "NOOB SHOOTER";
//         // let size = 30.0;
//         // let dim = measure_text(text, None, size as u16, 1.0);
//         // request_new_screen_size(x,y);
//         // clear_background(BLACK);
//         // draw_rectangle(50.0, 50.0, 100.0, 60.0, BLUE);
//         // draw_rectangle(200.0, 100.0, 120.0, 70.0, BLUE);
//         // draw_rectangle(400.0, 200.0, 80.0, 50.0, BLUE);
//         // draw_rectangle(600.0, 150.0, 150.0, 90.0, BLUE);
//         // draw_rectangle(800.0, 300.0, 100.0, 100.0, BLUE);
//         // draw_text("NOOB SHOOTER", (x-dim.width)/2.0, 20.0, 30.0, YELLOW);
//         // let mouse_pos = mouse_position();
//         // draw_circle(mouse_pos.0, mouse_pos.1, 10.0, color_c);
//         // match is_mouse_button_pressed(MouseButton::Left) {
//         //     true => color_c = GREEN,
//         //     false => ()
//         // }

//         // match is_mouse_button_released(MouseButton::Left) {
//         //     true => color_c = RED,
//         //     false => ()
//         // }
//         next_frame().await;

//     }
// }


use macroquad::prelude::*;

#[macroquad::main("3D Perspective Example")]
async fn main() {
    // Camera starts at (0, 0, -10), looking at the origin
    let mut camera = Camera3D {
        position: vec3(0.0, 0.0, -10.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(BLACK);

        set_camera(&camera);
        draw_cube(vec3(0.0, 0.0, 10.0), vec3(2.0, 2.0, 2.0), None, BLUE);


        // Move the camera forward (toward positive Z)
        if is_key_down(KeyCode::W) {
            camera.position.z += 0.2;
            camera.target.z += 0.2; // Keep looking forward
        }

        if is_key_down(KeyCode::S) {
            camera.position.z -= 0.2;
            camera.target.z -= 0.2;
        }

        const WORLD_SCALE: f32 = 100.0;
        let mouse_pos = mouse_position();
        let window_size = (screen_width(), screen_height());
        let world_x = (mouse_pos.0 / window_size.0 as f32) * WORLD_SCALE - (WORLD_SCALE / 2.0);
        let world_y = (mouse_pos.1 / window_size.1 as f32) * WORLD_SCALE - (WORLD_SCALE / 2.0);

        camera.position.x = world_x;
        camera.position.y = world_y;


        set_default_camera(); // for UI or 2D overlays
        draw_text("Use W/S to move forward/backward", 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}
