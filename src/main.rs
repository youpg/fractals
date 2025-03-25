use macroquad::prelude::*;
use std::fs;

async fn load_shader(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Failed to load shader {}: {}", path, e))
}

#[macroquad::main("Mandelbrot Set")]
async fn main() {
    println!("Loading shaders...");

    let vert_shader = match load_shader("shaders/mandelbrot.vert").await {
        Ok(shader) => {
            println!("✅ Vertex shader loaded successfully");
            shader
        }
        Err(e) => {
            println!("❌ Error loading vertex shader: {}", e);
            return;
        }
    };

    let frag_shader = match load_shader("shaders/mandelbrot.frag").await {
        Ok(shader) => {
            println!("✅ Fragment shader loaded successfully");
            shader
        }
        Err(e) => {
            println!("❌ Error loading fragment shader: {}", e);
            return;
        }
    };

    println!("Creating material...");
    let material = match load_material(
        ShaderSource::Glsl {
            vertex: &vert_shader,
            fragment: &frag_shader,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("u_view_min", UniformType::Float2),
                UniformDesc::new("u_view_max", UniformType::Float2),
                UniformDesc::new("u_screen_size", UniformType::Float2),
            ],
            ..Default::default()
        },
    ) {
        Ok(mat) => {
            println!("✅ Material created successfully");
            mat
        }
        Err(e) => {
            println!("❌ Error creating material: {}", e);
            return;
        }
    };

    println!("Starting Mandelbrot visualization...");

    let mut view_min = Vec2::new(-2.0, -1.5);
    let mut view_max = Vec2::new(1.0, 1.5);
    let mut prev_mouse_pos: Option<Vec2> = None;

    loop {
        let screen_size = vec2(screen_width(), screen_height());

        // Handle input for zooming with mouse wheel
        let (_, mouse_wheel_y) = mouse_wheel();
        if mouse_wheel_y != 0.0 {
            let zoom_factor = if mouse_wheel_y > 0.0 { 0.9 } else { 1.1 };
            let (mouse_x, mouse_y) = mouse_position();

            let target_x = view_min.x + (mouse_x / screen_size.x) * (view_max.x - view_min.x);
            let target_y = view_min.y + (mouse_y / screen_size.y) * (view_max.y - view_min.y);

            view_min.x = target_x - (target_x - view_min.x) * zoom_factor;
            view_max.x = target_x + (view_max.x - target_x) * zoom_factor;

            view_min.y = target_y - (target_y - view_min.y) * zoom_factor;
            view_max.y = target_y + (view_max.y - target_y) * zoom_factor;
        }

        // Handle input for panning with mouse drag
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let current_mouse_pos = Vec2::new(mouse_x, mouse_y);
            if let Some(prev) = prev_mouse_pos {
                let delta = current_mouse_pos - prev;
                let view_size = view_max - view_min;

                let delta_complex = Vec2::new(
                    delta.x * (view_size.x / screen_size.x),
                    -delta.y * (view_size.y / screen_size.y),
                );

                let delta_view = Vec2::new(-delta_complex.x, delta_complex.y);
                view_min += delta_view;
                view_max += delta_view;
            }
            prev_mouse_pos = Some(current_mouse_pos);
        } else {
            prev_mouse_pos = None;
        }

        // Reset view on 'R' key press
        if is_key_pressed(KeyCode::R) {
            view_min = Vec2::new(-2.0, -1.5);
            view_max = Vec2::new(1.0, 1.5);
        }

        // Update shader uniforms and draw mandelbrot
        gl_use_material(&material);
        material.set_uniform("u_view_min", view_min);
        material.set_uniform("u_view_max", view_max);
        material.set_uniform("u_screen_size", screen_size);

        draw_rectangle(0.0, 0.0, screen_size.x, screen_size.y, WHITE);
        gl_use_default_material();

        // Draw FPS count 
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}