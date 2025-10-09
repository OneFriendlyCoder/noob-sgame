mod utilis;
mod enemy;
mod collision;
mod player;
mod grid;
mod infinity;
mod camera;

use macroquad::prelude::*;
use utilis::*;
use enemy::*;
use collision::*;
use player::*;
use grid::*;
use camera::*;
use infinity::*;

#[macroquad::main("RUSTY KRUNKER")]
async fn main() {
    set_pc_assets_folder("./assets/");
    let (screen_h, screen_w, screen_d) = board_size();
    let road_half = 0.01;
    let lane_half = 0.005;
    let grass_half = 0.09;
    let red_half = 0.32;
    let x_min = -screen_w * (road_half + lane_half + grass_half + red_half);
    let x_max =  screen_w * (road_half + lane_half + grass_half + red_half);
    let y_min = -screen_h / 2.0;
    let y_max =  screen_h / 2.0;
    let z_min = -screen_d / 2.0;
    let z_max =  screen_d / 2.0;

    const MOUSE_SENSITIVITY: f32 = 0.005;
    let mut best_score = 0.0;
    let mut player: Player = Player::new(vec3(0.0, screen_h * 0.001, 0.0),
                                        vec3(0.0, 0.0, 0.0),
                                        "Player1".to_string(),
                                        "Shotgun".to_string(),
                                        0.0, 0.0);
    let mut camera = Camera3D {
        position: player.position,
        target: player.target,
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };
    let mut camera1 = Camera3D {
        position: player.position,
        target: player.position,
        fovy: 90.0,
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };
    let texture: Texture2D = load_texture("textures/crosshair.png").await.unwrap();
    let mut start_time = get_time();
    let game_duration = 30.0;
    async fn reset_game(player: &mut Player, enemies: &mut Enemies, grid: &mut Grid, start_time: &mut f64, x_min: f32, x_max: f32, z_min: f32, z_max: f32) {
        player.total_points = 0;
        player.targets_shot = 0;
        player.position = vec3(0.0, 0.0, 0.0);
        *enemies = Enemies::init_enemies(100, x_min, x_max, z_min, z_max).await;
        *grid = init_grid(enemies, x_min, x_max, z_min, z_max, 10, 10);

        *start_time = get_time();
    }
    let mut enemies = Enemies::init_enemies(100, x_min, x_max, z_min, z_max).await;
    let mut grid = init_grid(&enemies, x_min, x_max, z_min, z_max, 10, 10);
    let mut camera_view = CameraView::FirstPerson;
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::R) {
            reset_game(&mut player, &mut enemies, &mut grid, &mut start_time, x_min, x_max, z_min, z_max).await;
        }
        let delta_time = get_frame_time();
        enemies.update_enemies_grid(delta_time, &mut grid);
        if is_key_down(KeyCode::V) {
            camera_view = CameraView::ThirdPerson;
            set_camera(&camera1);
        } else {
            camera_view = CameraView::FirstPerson;
            set_camera(&camera);
        }
        show_mouse(false);
        draw_cube(
            vec3(0.0, -1.0, 0.0),
            vec3(screen_w * (road_half*2.0 + 2.0*lane_half + 2.0*grass_half + 2.0*red_half), 0.1, screen_d),
            None,
            DARKGRAY,
        );
        player.draw_player(&camera_view);
        let screen_center = vec2(screen_w / 2.0, screen_h / 2.0);
        let mouse_pos = vec2(mouse_position().0, mouse_position().1);
        let offset = mouse_pos - screen_center;
        player.yaw = -(offset.x) * MOUSE_SENSITIVITY;
        player.pitch = -(offset.y) * MOUSE_SENSITIVITY;
        let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.1;
        player.pitch = player.pitch.clamp(-pitch_limit, pitch_limit);
        let look = vec3(player.yaw.cos()*player.pitch.cos(), player.pitch.sin(), player.yaw.sin()*player.pitch.cos());
        let forward = vec3(player.yaw.cos(), 0.0, player.yaw.sin());
        let strafe_dir = vec3(-forward.z, 0.0, forward.x);
        player.update_player_position(forward, strafe_dir, look, &mut enemies, &mut grid, &mut camera,&mut camera1 ,camera_view);
        enemies.draw_enemies();

        set_default_camera(); // for 2D UI
        let size = vec2(texture.width()/20.0, texture.height()/20.0);
        draw_texture_ex(&texture, (screen_width()-size.x)/2.0, (screen_height()-size.y)/2.0, RED, DrawTextureParams{dest_size: Some(size), ..Default::default()});

        let elapsed_time = get_time() - start_time;
        let time_remaining = (game_duration - elapsed_time).max(0.0);

        if player.total_points > player.best_score {
            player.best_score = player.total_points;
        }
        let ui_text = format!(
            "Score: {}\nBest Score: {}\nTime Remaining: {:.0}s\nPress R to Reset",
            player.total_points,
            player.best_score,
            time_remaining
        );
        draw_text_ex(
            &ui_text,
            20.0,
            40.0,
            TextParams {
                font_size: 25,
                color: WHITE,
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
