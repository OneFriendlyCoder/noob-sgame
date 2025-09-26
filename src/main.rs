use macroquad::prelude::*;

#[macroquad::main("NOOB's GAME")]
async fn main() {
    set_fullscreen(true);

    let screen_h: f32 = screen_height();
    let screen_w: f32 = screen_width();
    let screen_d: f32 = screen_h.max(screen_w);
    let road_half = 0.01;
    let lane_half = 0.005;
    let grass_half = 0.09;
    let red_half = 0.32;

    let camera = Camera3D {
        position: vec3(0.0, screen_h*0.01, 0.0),
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

        next_frame().await;
    }
}
