use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut color_c:Color = RED;
    loop {
        let x = 1000.0;
        let y = 700.0;
        let text = "NOOB SHOOTER";
        let size = 30.0;
        let dim = measure_text(text, None, size as u16, 1.0);
        request_new_screen_size(x,y);
        clear_background(BLACK);
        draw_rectangle(50.0, 50.0, 100.0, 60.0, BLUE);
        draw_rectangle(200.0, 100.0, 120.0, 70.0, BLUE);
        draw_rectangle(400.0, 200.0, 80.0, 50.0, BLUE);
        draw_rectangle(600.0, 150.0, 150.0, 90.0, BLUE);
        draw_rectangle(800.0, 300.0, 100.0, 100.0, BLUE);
        draw_text("NOOB SHOOTER", (x-dim.width)/2.0, 20.0, 30.0, YELLOW);
        let mouse_pos = mouse_position();
        draw_circle(mouse_pos.0, mouse_pos.1, 10.0, color_c);
        match is_mouse_button_pressed(MouseButton::Left) {
            true => color_c = GREEN,
            false => ()
        }

        match is_mouse_button_released(MouseButton::Left) {
            true => color_c = RED,
            false => ()
        }
        next_frame().await;

    }
}

