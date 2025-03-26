use macroquad::prelude::*;
use std::fs;
use egui_macroquad::{self, egui};

// load GLSL shader
async fn load_shader(shader_path: &str) -> Result<String, String> {
    fs::read_to_string(shader_path)
        .map_err(|e| format!("Failed to load shader {}: {}", shader_path, e))
}

#[macroquad::main("Mandelbrot Set Visualizer")]
async fn main() {
    // shader paths
    let vert_shader_path = "shaders/mandelbrot.vert";
    let frag_shader_path = "shaders/mandelbrot.frag";
    
    // load the vert and frag shader
    let vert_shader = load_shader(vert_shader_path).await.unwrap_or_else(|e| {
        eprintln!("❌ Vertex shader error: {}", e);
        std::process::exit(1);
    });
    
    let frag_shader = load_shader(frag_shader_path).await.unwrap_or_else(|e| {
        eprintln!("❌ Fragment shader error: {}", e);
        std::process::exit(1);
    });

    // var init
    let mut viewport_min = Vec2::new(-2.0, -1.5);
    let mut viewport_max = Vec2::new(1.0, 1.5);
    let mut escape_radius: f32 = 4.0;
    let mut max_iterations: i32 = 1000;
    let mut prev_mouse_position: Option<Vec2> = None;

    // create shader material for mandelbrot
    let material = load_material(
        &vert_shader,
        &frag_shader,
        MaterialParams {
            uniforms: vec![
                ("u_viewport_min".to_string(), UniformType::Float2),
                ("u_viewport_max".to_string(), UniformType::Float2),
                ("u_screen_dimensions".to_string(), UniformType::Float2),
                ("u_max_iterations".to_string(), UniformType::Int1),
                ("u_escape_radius".to_string(), UniformType::Float1),
            ],
            ..Default::default()
        },
    ).expect("Failed to create shader material");

    loop {
        let screen_dimensions = vec2(screen_width(), screen_height());

        // load egui control panel
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Fractal Controls").show(ctx, |ui| {
                ui.label("Render Quality");
                ui.add(egui::Slider::new(&mut max_iterations, 0..=5000).logarithmic(true));
                ui.label("Escape Radius");
                ui.add(egui::Slider::new(&mut escape_radius, 2.0..=8.0).step_by(0.1));
            });
        });

        handle_zoom(&mut viewport_min, &mut viewport_max, screen_dimensions);
        handle_panning(&mut viewport_min, &mut viewport_max, screen_dimensions, &mut prev_mouse_position);

        // reset viewport on 'R'
        if is_key_pressed(KeyCode::R) {
            viewport_min = Vec2::new(-2.0, -1.5);
            viewport_max = Vec2::new(1.0, 1.5);
        }

        // update uniforms
        gl_use_material(material);
        material.set_uniform("u_viewport_min", viewport_min);
        material.set_uniform("u_viewport_max", viewport_max);
        material.set_uniform("u_screen_dimensions", screen_dimensions);
        material.set_uniform("u_max_iterations", max_iterations);
        material.set_uniform("u_escape_radius", escape_radius);

        // render mandelbrot
        draw_rectangle(0.0, 0.0, screen_dimensions.x, screen_dimensions.y, WHITE);
        gl_use_default_material();

        // display FPS
        draw_text(
            &format!("FPS: {}\nIterations: {}", get_fps(), max_iterations),
            20.0,
            40.0,
            24.0,
            Color::new(1.0, 1.0, 1.0, 0.7),
        );

        egui_macroquad::draw();
        next_frame().await;
    }
}

/// handle mouse zoom with scroll wheel
fn handle_zoom(viewport_min: &mut Vec2, viewport_max: &mut Vec2, screen_dimensions: Vec2) {
    let (_, mouse_wheel_y) = mouse_wheel();
    if mouse_wheel_y == 0.0 {
        return;
    }

    let zoom_factor = if mouse_wheel_y > 0.0 { 0.9 } else { 1.1 };
    let (mouse_x, mouse_y) = mouse_position();

    let target_x = viewport_min.x + (mouse_x / screen_dimensions.x) * (viewport_max.x - viewport_min.x);
    let target_y = viewport_min.y + (mouse_y / screen_dimensions.y) * (viewport_max.y - viewport_min.y);


    *viewport_min = Vec2::new(
        target_x - (target_x - viewport_min.x) * zoom_factor,
        target_y - (target_y - viewport_min.y) * zoom_factor,
    );
    
    *viewport_max = Vec2::new(
        target_x + (viewport_max.x - target_x) * zoom_factor,
        target_y + (viewport_max.y - target_y) * zoom_factor,
    );
}

/// handle panning with space + mouse drag
fn handle_panning(
    viewport_min: &mut Vec2,
    viewport_max: &mut Vec2,
    screen_dimensions: Vec2,
    prev_mouse_position: &mut Option<Vec2>,
) {
    let space_pressed = is_key_down(KeyCode::Space);
    let mouse_down = is_mouse_button_down(MouseButton::Left);

    if space_pressed && mouse_down {
        let current_pos = Vec2::from(mouse_position());

        if let Some(previous_pos) = *prev_mouse_position {
            let delta = current_pos - previous_pos;
            let view_size = *viewport_max - *viewport_min;

            // Convert screen delta to complex plane delta
            let complex_delta = Vec2::new(
                delta.x * (view_size.x / screen_dimensions.x),
                delta.y * (view_size.y / screen_dimensions.y), // Invert Y axis
            );

            *viewport_min -= complex_delta;
            *viewport_max -= complex_delta;
        }

        *prev_mouse_position = Some(current_pos);
    } else {
        *prev_mouse_position = None;
    }
}