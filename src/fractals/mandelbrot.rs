use egui_macroquad::egui;
use crate::fractals::fractal::{Fractal, Shader, FractalData};

pub struct Mandelbrot {
    pub data: FractalData,
}

#[async_trait::async_trait]
impl Fractal for Mandelbrot {
    async fn new(shader: &Shader) -> Self {
        let fractal_data = FractalData::new(shader.clone()).await;
        Self {
            data: fractal_data,
        }
    }

    fn add_specific_ui_elements(&mut self, ui: &mut egui::Ui) { }

    fn data_mut(&mut self) -> &mut FractalData {
        &mut self.data
    }
}
