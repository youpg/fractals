use macroquad::prelude::*;
use egui_macroquad::egui;


mod fractals;
use fractals::{
    fractal::Fractal,
    fractal::FractalType, 
    fractal::Shader,

    // Fractals
    mandelbrot::Mandelbrot,
    julia::Julia,
};

#[macroquad::main("Fractal Visualizer")]
async fn main() {
    let fractal_type: FractalType = FractalType::Julia;

    let mut fractal: Box<dyn Fractal> = match fractal_type {
        FractalType::Mandelbrot => Box::new(Mandelbrot::new(&Shader { 
            vertex: "shaders/mandelbrot.vert".to_string(), 
            fragment: "shaders/mandelbrot.frag".to_string() 
        }).await),
        FractalType::Julia => Box::new(Julia::new(&Shader { 
            vertex: "shaders/julia.vert".to_string(), 
            fragment: "shaders/julia.frag".to_string() 
        }).await),
    };

    let mut prev_mouse_position: Option<Vec2> = None;

    loop {
        let screen_dimensions: Vec2 = vec2(screen_width(), screen_height());
        fractal.update(get_frame_time());

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Fractal Controls").show(ctx, |ui| {
                fractal.add_basic_ui_elements(ui);
            });
        });


        // handle user interactions
        fractal.handle_zoom(screen_dimensions);
        fractal.handle_panning(screen_dimensions, &mut prev_mouse_position);
        if is_key_pressed(KeyCode::R) { fractal.reset_viewport(); }
        
        fractal.render(screen_dimensions);

        draw_text(
            &format!("FPS: {}", get_fps()),
            20.0,
            40.0,
            24.0,
            Color::new(1.0, 1.0, 1.0, 0.7),
        );

        egui_macroquad::draw();
        next_frame().await;
    }
}