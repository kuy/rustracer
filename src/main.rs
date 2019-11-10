mod coordinate;
mod point;
mod vector;

use crate::point::Point3D;
use crate::vector::{General, Normal, Vector3D};
use piston_window::*;

struct Sphere {
    center: Point3D,
    radius: f32,
}

struct Line {
    origin: Point3D,
    dir: Vector3D<Normal>,
}

struct Plane {
    origin: Point3D,
    dir: Vector3D<Normal>,
}

struct Dim2D {
    width: f32,
    height: f32,
}

struct Canvas {
    plane: Plane,
    dim: Dim2D,
}

impl Canvas {
    fn new(x: f32, y: f32, z: f32, w: f32, h: f32) -> Canvas {
        let origin = Point3D { x, y, z };
        let dir = Vector3D::<Normal>::from(x, y, z);
        let dim = Dim2D {
            width: w,
            height: h,
        };
        Canvas {
            plane: Plane { origin, dir },
            dim,
        }
    }
}

fn main() {
    let camera = Point3D {
        x: 50.0,
        y: 10.0,
        z: 150.0,
    };

    let light = Point3D {
        x: 50.0,
        y: 100.0,
        z: 75.0,
    };

    let v1 = Vector3D::<General>::new(10.0, 20.0, 30.0);
    println!("v1={:?}", v1);

    let l1 = v1.length();
    println!("v1.len={}", l1);

    let n1 = v1.norm();
    println!("n1={:?}", n1);

    let l2 = n1.length();
    println!("n1.len={}", l2);

    let mut window: PistonWindow = WindowSettings::new("rustracer", [100, 100])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, g, _device| {
            clear([1.0; 4], g);
            rectangle(
                [0.0, 0.0, 1.0, 1.0],
                [25.0, 25.0, 1.0, 1.0],
                ctx.transform,
                g,
            );
        });
    }
}
