use crate::canvas::Canvas;
use crate::coordinate::{self as coord, Coordinate};
use crate::vector::{General, Vector3D};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Point2D<T: Coordinate> {
    pub x: f32,
    pub y: f32,
    _c: PhantomData<T>,
}

impl<T: Coordinate> Point2D<T> {
    pub fn screen_at(x: i32, y: i32) -> Point2D<coord::Screen> {
        Point2D::<coord::Screen> {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5,
            _c: PhantomData,
        }
    }
}

impl Point2D<coord::Screen> {
    pub fn convert(&self, canvas: &Canvas) -> Point3D {
        let lx = self.x - canvas.dim.width / 2.0;
        let ly = self.y - canvas.dim.height / 2.0;
        Point3D::new(
            canvas.plane.origin.x + lx,
            canvas.plane.origin.y + ly,
            canvas.plane.origin.z,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn to(&self, dest: &Self) -> Vector3D<General> {
        Vector3D::<General>::new(dest.x - self.x, dest.y - self.y, dest.z - self.z)
    }
}
