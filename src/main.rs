use macroquad::prelude::*;
use std::fs;

async fn load_shader(path: &str) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Failed to load shader {}: {}", path, e))
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

    // Create material with error handling
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

    loop {
        let screen_size = vec2(screen_width(), screen_height());

        gl_use_material(&material);
        material.set_uniform("u_view_min", vec2(-2.0, -1.5));
        material.set_uniform("u_view_max", vec2(1.0, 1.5));
        material.set_uniform("u_screen_size", screen_size);

        draw_rectangle(0.0, 0.0, screen_size.x, screen_size.y, WHITE);

        gl_use_default_material();


        draw_text(&format!("FPS: {}", get_fps()), 20.0, 40.0, 30.0, WHITE);
        next_frame().await;
    }
}