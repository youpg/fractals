use macroquad::prelude::*;

const LAVENDER_COLOR: Color = Color::new(199.0 / 255.0, 161.0 / 255.0, 200.0 / 255.0, 1.0);
const MAX_ITER_NUMBER: u32 = 100;

struct Vec2f64 {
    x: f64,
    y: f64,
}

struct ColorStop {
    position: f32,
    color: Color,
}

// Linearly interpolates between two values
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

// Linearly interpolates between two colors
fn lerp_color(c1: Color, c2: Color, t: f32) -> Color {
    Color::new(
        lerp(c1.r, c2.r, t),
        lerp(c1.g, c2.g, t),
        lerp(c1.b, c2.b, t),
        1.0,
    )
}

// Returns a smooth gradient color based on Mandelbrot iteration count
fn get_color(iterations: u32) -> Color {
    let t = iterations as f32 / MAX_ITER_NUMBER as f32;

    let colors = [
        ColorStop { position: 0.0, color: Color::new(0.0 / 255.0, 7.0 / 255.0, 100.0 / 255.0, 1.0) },  // Dark Blue
        ColorStop { position: 0.16, color: Color::new(32.0 / 255.0, 107.0 / 255.0, 203.0 / 255.0, 1.0) }, // Blue
        ColorStop { position: 0.42, color: Color::new(237.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 1.0) }, // White
        ColorStop { position: 0.6425, color: Color::new(255.0 / 255.0, 170.0 / 255.0, 0.0 / 255.0, 1.0) }, // Orange
        ColorStop { position: 0.8575, color: Color::new(0.0 / 255.0, 2.0 / 255.0, 0.0 / 255.0, 1.0) }, // Almost Black
    ];

    for i in 0..colors.len() - 1 {
        let c1 = &colors[i];
        let c2 = &colors[i + 1];

        if t >= c1.position && t < c2.position {
            let blend = (t - c1.position) / (c2.position - c1.position);
            return lerp_color(c1.color, c2.color, blend);
        }
    }

    colors.last().unwrap().color
}

// Computes the Mandelbrot iteration count for a given point
fn calculate_mandelbrot(c: Vec2f64) -> u32 {
    let (mut x, mut y) = (0.0, 0.0);
    let mut iterations = 0;

    while x * x + y * y <= 4.0 && iterations < MAX_ITER_NUMBER {
        let x_new = x * x - y * y + c.x;
        let y_new = 2.0 * x * y + c.y;
        x = x_new;
        y = y_new;
        iterations += 1;
    }
    
    iterations
}

#[macroquad::main("Mandelbrot Set")]
async fn main() {
    let (screen_width, screen_height) = (screen_width(), screen_height());

    let min_x = -2.0;
    let max_x = 1.0;
    let min_y = -1.5;
    let max_y = 1.5;

    loop {
        clear_background(LAVENDER_COLOR);

        for px in 0..screen_width as u32 {
            for py in 0..screen_height as u32 {
                let c_x = min_x + (px as f64 / screen_width as f64) * (max_x - min_x);
                let c_y = min_y + (py as f64 / screen_height as f64) * (max_y - min_y);
                let c = Vec2f64 { x: c_x, y: c_y };

                let iterations = calculate_mandelbrot(c);
                let color = if iterations == MAX_ITER_NUMBER {
                    Color::new(0.0, 0.0, 0.0, 1.0)
                } else {
                    get_color(iterations)
                };

                draw_rectangle(px as f32, py as f32, 1.0, 1.0, color);
            }
        }

        next_frame().await;
    }
}
