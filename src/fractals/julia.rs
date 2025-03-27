use macroquad::prelude::Vec2;
use egui_macroquad::egui;
use crate::fractals::fractal::{Fractal, Shader, FractalData};

pub struct Julia {
    pub data: FractalData,
    pub a: f32,
    animating: bool,
    animation_speed: f32,
}

#[async_trait::async_trait]
impl Fractal for Julia {
    async fn new(shader: &Shader) -> Self {
        let fractal_data: FractalData = FractalData::new(shader.clone()).await;
        Self {
            data: fractal_data,
            a: 0.0,
            animating: false,
            animation_speed: std::f32::consts::PI/4.0, // rad / sec
        }
    }

    fn update(&mut self, delta_time: f32) {
        if self.animating {
            self.a += delta_time * self.animation_speed;
            self.a %= std::f32::consts::TAU;
        }
    }

    fn add_specific_ui_elements(&mut self, ui: &mut egui::Ui) { 
        ui.label("Julia Angle (a)");
        ui.add(egui::Slider::new(&mut self.a, 0.0..=std::f32::consts::TAU).text("Angle"));

        if ui.button("Toggle Animation").clicked() {
            self.animating = !self.animating;
        }
    }

    fn add_extra_uniforms(&mut self) {
        let a: f32 =  self.a;
        let data: &mut FractalData = self.data_mut();
        data.material.set_uniform("u_julia_a", a);
    }

    fn reset_viewport(&mut self) {
        self.data.viewport_min = Vec2::new(-1.5, -1.5);
        self.data.viewport_max = Vec2::new(1.5, 1.5);
    }

    fn data_mut(&mut self) -> &mut FractalData {
        &mut self.data
    }
}
