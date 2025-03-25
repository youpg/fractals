use macroquad::prelude::*;
const LAVENDER_COLOR: Color = Color::new(199.0/255.0, 161.0/255.0, 200.0/255.0, 1.0);
const MAX_ITER_NUMBER: u32 = 50;

struct Point {
    pub x: f64,
    pub y: f64,
}


fn calculate_mandelbrot_for_pixel(c: Point) -> u32 {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut iterations = 0;


    while x*x + y*y <= 4.0 && iterations <= MAX_ITER_NUMBER-1 {
        let x_new: f64 = x*x - y*y + c.x;
        let y_new: f64 = 2.0*x*y + c.y;

        x = x_new;
        y = y_new;
        iterations += 1;
        // println!("Point for i = {}: ({}, {})", iterations, x, y);
    }
    
    iterations
}



#[macroquad::main("Fractals")]
async fn main() {
    let screen_width = screen_width();
    let screen_height = screen_height();
    

    let min_x  = -2.0;
    let max_x = 1.0;
    let min_y  = -1.5;
    let max_y = 1.5;



    
    loop {
        clear_background(LAVENDER_COLOR);

        for px in 0..screen_width as u32 {
            for py in 0..screen_height as u32 {
                let c_x = min_x + (px as f64 / screen_width as f64) * (max_x - min_x);
                let c_y = min_y + (py as f64 / screen_height as f64) * (max_y - min_y);
                let c = Point { x: c_x, y: c_y };

                let iterations = calculate_mandelbrot_for_pixel(c);

                let color = if iterations == MAX_ITER_NUMBER {
                    Color::new(0.0, 0.0, 0.0, 1.0)
                } else {
                    let t = iterations as f32 / MAX_ITER_NUMBER as f32;
                    Color::new(t, t * 0.5, 1.0-t, 1.0)
                };

                draw_rectangle(px as f32, py as f32, 1.0, 1.0, color);
            }
        }
        
       
        next_frame().await
    }
}
