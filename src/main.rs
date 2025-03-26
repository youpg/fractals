use macroquad::prelude::*;
use std::fs;
use egui_macroquad;
use egui_macroquad::egui; // Import the re-exported egui

async fn load_shader(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Failed to load shader {}: {}", path, e))
}

#[macroquad::main("Mandelbrot Set with egui")]
async fn main() {
    println!("Loading shaders...");

    // Load shaders
    let vert_shader = match load_shader("shaders/mandelbrot.vert").await {
        Ok(shader) => shader,
        Err(e) => {
            eprintln!("❌ Error loading vertex shader: {}", e);
            return;
        }
    };

    let frag_shader = match load_shader("shaders/mandelbrot.frag").await {
        Ok(shader) => shader,
        Err(e) => {
            eprintln!("❌ Error loading fragment shader: {}", e);
            return;
        }
    };

    println!("Creating material...");
    // Create material with the uniform for max iterations
    let material = load_material(
        &vert_shader,
        &frag_shader,
        MaterialParams {
            uniforms: vec![
                ("u_view_min".to_string(), UniformType::Float2),
                ("u_view_max".to_string(), UniformType::Float2),
                ("u_screen_size".to_string(), UniformType::Float2),
                ("u_max_iterations".to_string(), UniformType::Int1),
            ],
            ..Default::default()
        },
    )
    .expect("Failed to create material");

    println!("Starting Mandelbrot visualization...");

    let mut view_min = Vec2::new(-2.0, -1.5);
    let mut view_max = Vec2::new(1.0, 1.5);
    let mut prev_mouse_pos: Option<Vec2> = None;
    // Initial maximum iterations value
    let mut max_iterations: i32 = 1000;

    loop {
        let screen_size = vec2(screen_width(), screen_height());

        // egui integration for the slider UI
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Settings").show(ctx, |ui| {
                ui.label("Max Iterations");
                ui.add(egui::Slider::new(&mut max_iterations, 1..=500));
            });
        });

        // Handle zoom with mouse wheel
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

        // Handle panning with mouse drag
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let current_mouse_pos = Vec2::new(mouse_x, mouse_y);
            
            if let Some(prev) = prev_mouse_pos {
                let delta = current_mouse_pos - prev;
                let view_size = view_max - view_min;

                let delta_complex = Vec2::new(
                    delta.x * (view_size.x / screen_size.x),
                    delta.y * (view_size.y / screen_size.y),
                );

                view_min -= delta_complex;
                view_max -= delta_complex;
            }
            prev_mouse_pos = Some(current_mouse_pos);
        } else {
            prev_mouse_pos = None;
        }

        // Reset view on R key
        if is_key_pressed(KeyCode::R) {
            view_min = Vec2::new(-2.0, -1.5);
            view_max = Vec2::new(1.0, 1.5);
        }

        // Update shader uniforms
        gl_use_material(material);
        material.set_uniform("u_view_min", view_min);
        material.set_uniform("u_view_max", view_max);
        material.set_uniform("u_screen_size", screen_size);
        material.set_uniform("u_max_iterations", max_iterations);

        // Draw fullscreen quad
        draw_rectangle(0.0, 0.0, screen_size.x, screen_size.y, WHITE);
        gl_use_default_material();

        // Draw FPS counter
        draw_text(
            &format!("FPS: {}", get_fps()),
            20.0,
            40.0,
            30.0,
            Color::new(1.0, 1.0, 1.0, 0.5),
        );


        egui_macroquad::draw();
        next_frame().await;
    }
}
