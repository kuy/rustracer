mod canvas;
mod coordinate;
mod dim;
mod line;
mod plane;
mod point;
mod sphere;
mod vector;

use crate::canvas::Canvas;
use crate::point::{Point2D, Point3D};
use crate::sphere::Sphere;
use ::image::{ImageBuffer, Rgba};
use piston_window::*;

const W: usize = 200;
const H: usize = 200;
const TOTAL: usize = W * H;

fn main() {
    let canvas = Canvas::new(50.0, 50.0, 75.0, W as f32, H as f32);
    let camera = Point3D::new(50.0, 50.0, 150.0);
    let obj1 = Sphere::new(50.0, 20.0, 50.0, 20.0);

    let mut window: PistonWindow = WindowSettings::new("rustracer", [W as u32, H as u32])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_lazy(true);

    let mut frame = ImageBuffer::new(W as u32, H as u32);
    let mut context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let settings = TextureSettings::new();
    let mut texture: G2dTexture = Texture::from_image(&mut context, &frame, &settings).unwrap();

    // Raytracing
    for i in 0..TOTAL {
        let x = i % W;
        let y = (i - x) / W;
        let pos = Point2D::screen_at(x as i32, y as i32);
        let ray = canvas.cast_ray(&pos, &camera);
        match obj1.intersection(&ray) {
            Some(_) => frame.put_pixel(x as u32, (H - y) as u32, Rgba([255, 255, 255, 255])),
            _ => (),
        }
    }

    while let Some(event) = window.next() {
        texture.update(&mut context, &frame).unwrap();
        window.draw_2d(&event, |c, g, device| {
            context.encoder.flush(device);
            clear(color::BLACK, g);
            image(&texture, c.transform, g);
        });
    }
}
