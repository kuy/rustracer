mod canvas;
mod coordinate;
mod dim;
mod line;
mod plane;
mod point;
mod sphere;
mod vector;

use crate::canvas::Canvas;
use crate::coordinate as coord;
use crate::point::{Point2D, Point3D};
use crate::sphere::Sphere;
use piston_window::*;

fn main() {
    let canvas = Canvas::new(50.0, 50.0, 75.0, 100.0, 100.0);
    let camera = Point3D::new(50.0, 50.0, 150.0);
    let obj1 = Sphere::new(50.0, 20.0, 50.0, 20.0);

    let width: usize = 100;
    let height: usize = 100;
    let mut buf = vec![];

    let mut window: PistonWindow = WindowSettings::new("rustracer", [width as u32, height as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let black = [0.0, 0.0, 0.0, 1.0];
    let white = [1.0, 1.0, 1.0, 1.0];

    println!("Rendering...");
    for i in 0..10000 {
        // Raytracing
        // let i = buf.len();
        let x = i % width;
        let y = (i - x) / width;
        let pos = Point2D::<coord::Screen>::screen_at(x as i32, y as i32);
        let ray = canvas.cast_ray(&pos, &camera);

        let pixel = match obj1.intersection(&ray) {
            Some(_) => white,
            None => black,
        };
        buf.push(pixel);
    }
    println!("Done");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, g, _device| {
            // Rendering
            for x in 0..width {
                for y in 0..height {
                    let i = y * width + x;
                    let pixel = if i < buf.len() { buf[i] } else { black };
                    rectangle(pixel, [x as f64, y as f64, 1.0, 1.0], ctx.transform, g);
                }
            }
        });
    }
}
