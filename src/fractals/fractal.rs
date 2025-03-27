// fractal.rs
use macroquad::prelude::*;
use egui_macroquad::egui;

#[derive(PartialEq)]
pub enum FractalType {
    Mandelbrot,
    Julia,
}

#[derive(Clone)]
pub struct Shader {
    pub vertex: String,
    pub fragment: String,
}

#[async_trait::async_trait]
pub trait Fractal {
    async fn new(shader: &Shader) -> Self where Self: Sized;
    fn add_specific_ui_elements(&mut self, ui: &mut egui::Ui);
    fn add_extra_uniforms(&mut self) {}
    fn update(&mut self, _delta_time: f32) {}

    fn data_mut(&mut self) -> &mut FractalData;

    fn handle_zoom(&mut self, screen_dimensions: Vec2) {
        let (_, mouse_wheel_y) = mouse_wheel();
        if mouse_wheel_y == 0.0 {
            return;
        }
        let zoom_factor: f32 = if mouse_wheel_y > 0.0 { 0.9 } else { 1.1 };
        let (mouse_x, mouse_y) = mouse_position();
        let data = self.data_mut();

        let current_min = data.viewport_min;
        let current_max = data.viewport_max;

        let target_x = current_min.x + (mouse_x / screen_dimensions.x) * (current_max.x - current_min.x);
        let target_y = current_min.y + (mouse_y / screen_dimensions.y) * (current_max.y - current_min.y);

        data.viewport_min = Vec2::new(
            target_x - (target_x - current_min.x) * zoom_factor,
            target_y - (target_y - current_min.y) * zoom_factor,
        );

        data.viewport_max = Vec2::new(
            target_x + (current_max.x - target_x) * zoom_factor,
            target_y + (current_max.y - target_y) * zoom_factor,
        );
    }

    fn handle_panning(&mut self, screen_dimensions: Vec2, prev_mouse_position: &mut Option<Vec2>) {
        let space_pressed = is_key_down(KeyCode::Space);
        let mouse_down = is_mouse_button_down(MouseButton::Left);

        if space_pressed && mouse_down {
            let current_pos = Vec2::from(mouse_position());
            let data = self.data_mut();

            if let Some(previous_pos) = *prev_mouse_position {
                let delta = current_pos - previous_pos;
                let view_size = data.viewport_max - data.viewport_min;
                let complex_delta = Vec2::new(
                    delta.x * (view_size.x / screen_dimensions.x),
                    delta.y * (view_size.y / screen_dimensions.y),
                );

                data.viewport_min -= complex_delta;
                data.viewport_max -= complex_delta;
            }
            *prev_mouse_position = Some(current_pos);
        } else {
            *prev_mouse_position = None;
        }
    }

    fn reset_viewport(&mut self) {
        let data = self.data_mut();
        data.viewport_min = Vec2::new(-2.0, -1.5);
        data.viewport_max = Vec2::new(1.0, 1.5);
    }


    fn add_basic_ui_elements(&mut self, ui: &mut egui::Ui) {
        let data: &mut FractalData = self.data_mut();
        ui.label("Render Quality");
        ui.add(egui::Slider::new(&mut data.max_iterations, 0..=5000).logarithmic(true));
        ui.label("Escape Radius");
        ui.add(egui::Slider::new(&mut data.escape_radius, 0.0..=100.0).step_by(0.1));
        self.add_specific_ui_elements(ui);
    }

    fn render(&mut self, screen_dimensions: Vec2) {
        let data: &mut FractalData = self.data_mut();
        gl_use_material(data.material);
        data.material.set_uniform("u_viewport_min", data.viewport_min);
        data.material.set_uniform("u_viewport_max", data.viewport_max);
        data.material.set_uniform("u_screen_dimensions", screen_dimensions);
        data.material.set_uniform("u_max_iterations", data.max_iterations);
        data.material.set_uniform("u_escape_radius", data.escape_radius);
        self.add_extra_uniforms();

        
        draw_rectangle(0.0, 0.0, screen_dimensions.x, screen_dimensions.y, WHITE);
        gl_use_default_material();
    }
}





pub struct FractalData {
    pub shader: Shader,
    pub material: Material,
    pub viewport_min: Vec2,
    pub viewport_max: Vec2,
    pub escape_radius: f32,
    pub max_iterations: i32,
}

impl FractalData {
    pub async fn new(shader: Shader) -> Self {
        let material = crate::fractals::fractal::create_material(
            shader.vertex.as_str(),
            shader.fragment.as_str(),
        ).await;

        Self {
            shader,
            material,
            viewport_min: Vec2::new(-2.0, -1.5),
            viewport_max: Vec2::new(1.0, 1.5),
            escape_radius: 4.0,
            max_iterations: 1000,
        }
    }
}







pub async fn create_material(vertex_path: &str, fragment_path: &str) -> Material {
    let vertex_shader: String = load_shader(vertex_path).await.unwrap_or_else(|e|{
        eprintln!("❌ Vertex shader error: {}", e);
        std::process::exit(1);
    });

    let fragment_shader: String = load_shader(fragment_path).await.unwrap_or_else(|e|{
        eprintln!("❌ Fragment shader error: {}", e);
        std::process::exit(1);
    });

    let mut uniforms = vec![
        ("u_viewport_min".to_string(), UniformType::Float2),
        ("u_viewport_max".to_string(), UniformType::Float2),
        ("u_screen_dimensions".to_string(), UniformType::Float2),
        ("u_max_iterations".to_string(), UniformType::Int1),
        ("u_escape_radius".to_string(), UniformType::Float1),
    ];

    if fragment_path.contains("julia.frag") {
        uniforms.push(("u_julia_a".to_string(), UniformType::Float1));
    }

    let material_params: MaterialParams = MaterialParams { 
        uniforms,
        ..Default::default()
    };

    load_material(&vertex_shader, &fragment_shader, material_params).expect("Failed to create material")
}

async fn load_shader(shader_path: &str) -> Result<String, String> {
    std::fs::read_to_string(shader_path)
        .map_err(|e| format!("Failed to load shader {}: {}", shader_path, e))
}
