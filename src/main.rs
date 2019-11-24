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

const W: usize = 400;
const H: usize = 400;

fn main() {
    let canvas = Canvas::new(200.0, 200.0, 50.0, W as f32, H as f32);
    let camera = Point3D::new(200.0, 200.0, 200.0);
    let obj1 = Sphere::new(200.0, 25.0, 50.0, 25.0);
    let light = Point3D::new(200.0, 400.0, 50.0);

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
    for x in 0..W {
        for y in 0..H {
            let pos = Point2D::screen_at(x as i32, y as i32);
            let ray = canvas.cast_ray(&pos, &camera);
            match obj1.intersection(&ray) {
                Some(p) => {
                    let vl = light.to(&p).normalize();
                    let n = p.to(&obj1.center).normalize();
                    let dp1 = ray.dir.dot(&n);
                    let dp2 = vl.dot(&n);
                    let d = (dp1 - dp2).abs();
                    let c = (d * 255.0).round() as u8;
                    frame.put_pixel(x as u32, (H - y - 1) as u32, Rgba([c, c, c, 255]));
                }
                _ => (),
            }
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
