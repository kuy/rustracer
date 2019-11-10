use piston_window::*;
use std::marker::PhantomData;

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

type General = ();
type Normal = ();

#[derive(Debug)]
struct Vector3D<T> {
    x: f32,
    y: f32,
    z: f32,
    _t: PhantomData<T>,
}

impl<T> Vector3D<T> {
    fn new(x: f32, y: f32, z: f32) -> Vector3D<T> {
        Vector3D::<T> {
            x,
            y,
            z,
            _t: PhantomData,
        }
    }

    fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn norm(&self) -> Vector3D<Normal> {
        let len = self.length();
        Vector3D::new(self.x / len, self.y / len, self.z / len)
    }
}

struct Sphere {
    center: Point3D,
    radius: f32,
}

struct Line {
    origin: Point3D,
    dir: Vector3D<Normal>,
}

struct Plane {
    v: Vector3D<Normal>,
    d: f32,
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

    let v1 = Vector3D::<Normal>::new(10.0, 20.0, 30.0);
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
