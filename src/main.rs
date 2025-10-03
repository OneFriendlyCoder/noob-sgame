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


    // star draw, do not remove this code
    // let mut star_walls = vec![
    //     StarWall::new(1000, 0, x_min, x_max, y_min, y_max, z_min, z_max, 5.0),
    //     StarWall::new(1000, 1, x_min, x_max, y_min, y_max, z_min, z_max, 5.0),
    //     StarWall::new(1000, 2, x_min, x_max, y_min, y_max, z_min, z_max, 5.0),
    //     StarWall::new(1000, 3, x_min, x_max, y_min, y_max, z_min, z_max, 5.0),
    //     StarWall::new(1000, 4, x_min, x_max, y_min, y_max, z_min, z_max, 5.0),
    //     StarWall::new(1000, 5, x_min, x_max, y_min, y_max, z_min, z_max, 5.0),
    // ];


    const MOUSE_SENSITIVITY: f32 = 0.005;
    let mut player: Player = Player::new(vec3(0.0, screen_h * 0.001, 0.0),vec3(0.0, 0.0, 0.0) ,"Player1".to_string(), "Shotgun".to_string(), 0.0, 0.0);
    let mut camera = Camera3D {
        position: player.position,
        target: player.target,
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    // third person view camera
    let mut camera1 = Camera3D {
        position: player.position,
        target: player.position,
        fovy: 90.0,
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };
    
    let texture: Texture2D = load_texture("textures/crosshair.png").await.unwrap();
    let enemies = Enemies::init_enemies(100, x_min, x_max, z_min, z_max).await;
    let grid = init_grid(&enemies, x_min, x_max, z_min, z_max, 10, 10);
    let mut camera_view = CameraView::FirstPerson; 

    loop {
        clear_background(BLACK);

        // setting camera based on the person's view
        if is_key_down(KeyCode::V){
            camera_view = CameraView::ThirdPerson;
            set_camera(&camera1);
        } else {
            camera_view = CameraView::FirstPerson;
            set_camera(&camera);
        }

        // set_camera(&camera);
        show_mouse(false);
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

        //drawing player
        player.draw_player(&camera_view);

        // for wall in &star_walls {
        //     wall.draw();
        // }

        let screen_center = vec2(screen_w / 2.0, screen_h / 2.0); // screen_center : is the mid point of the gaming window
        let mouse_pos_tuple = mouse_position();
        let mouse_pos = vec2(mouse_pos_tuple.0, mouse_pos_tuple.1);
        let offset = mouse_pos - screen_center;

        // yaw, pitch and strafing done
        player.yaw = -(offset.x) * MOUSE_SENSITIVITY;
        player.pitch = -(offset.y) * MOUSE_SENSITIVITY;

        
        let pitch_limit = std::f32::consts::FRAC_PI_2 - 0.1;
        player.pitch = player.pitch.clamp(-pitch_limit, pitch_limit);
        let look = vec3(player.yaw.cos()*player.pitch.cos(), player.pitch.sin(), player.yaw.sin()*player.pitch.cos());
        let forward = vec3(player.yaw.cos(), 0.0, player.yaw.sin());
        let strafe_dir = vec3(-forward.z, 0.0, forward.x);
        player.update_player_position(forward, strafe_dir, look, &enemies, &grid, &mut camera,&mut camera1 ,camera_view);
        
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