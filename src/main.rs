use macroquad::prelude::*;
const LAVENDER_COLOR: Color = Color::new(199.0/255.0, 161.0/255.0, 200.0/255.0, 1.0);
const MAX_ITER_NUMBER: u32 = 1000;

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
        println!("Point for i = {}: ({}, {})", iterations, x, y);
    }
    
    iterations
}




#[macroquad::main("Fractals")]
async fn main() {
    let c: Point = Point { x: -0.5, y: -0.5 };

    
    calculate_mandelbrot_for_pixel(c);
    
    loop {
        clear_background(LAVENDER_COLOR);
        
       
        next_frame().await
    }
}
