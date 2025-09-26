use macroquad::prelude::*;

#[macroquad::main("NOOB's GAME")]
async fn main() {
    set_fullscreen(true);

    set_fullscreen(true);
    let screen_height:f32 = screen_height();
    let screen_width:f32 = screen_width();
    let screen_depth:f32 = screen_height.max(screen_width);
    let camera = Camera3D {
        position: vec3(0.0, 5.0, 0.0),
        target: vec3(0.0, 0.0, screen_depth),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    loop {
        clear_background(BLUE);
        set_camera(&camera);
        draw_cube(
            vec3(0.0, 0.0, screen_depth),
            vec3(4.0, 4.0, 4.0),
            None,
            RED,
        );

        draw_cube(
            vec3(0.0, -1.0, 0.0),
            vec3(12.0, 0.1, screen_depth),
            None,
            DARKGRAY,
        );

        draw_cube(
            vec3(-30.0, -1.0, 0.0),
            vec3(40.0, 0.1, screen_depth),
            None,
            GREEN,
        );

        draw_cube(
            vec3(30.0, -1.0, 0.0),
            vec3(40.0, 0.1, screen_depth),
            None,
            GREEN,
        );

        next_frame().await;
    }
}
