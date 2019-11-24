use crate::canvas::Canvas;
use crate::coordinate::{self as coord, Coordinate};
use crate::vector::{General, Kind, Vector3D};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Point2D<T: Coordinate> {
    pub x: f32,
    pub y: f32,
    _c: PhantomData<T>,
}

impl Point2D<coord::Screen> {
    pub fn screen_at(x: i32, y: i32) -> Point2D<coord::Screen> {
        Point2D::<coord::Screen> {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5,
            _c: PhantomData,
        }
    }

    pub fn convert(&self, canvas: &Canvas) -> Point3D {
        let lx = self.x - canvas.dim.width * 0.5;
        let ly = self.y - canvas.dim.height * 0.5;
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
        Vector3D::new(dest.x - self.x, dest.y - self.y, dest.z - self.z)
    }

    pub fn base<T: Kind>(&self, vec: &Vector3D<T>) -> Point3D {
        Point3D::new(self.x + vec.x, self.y + vec.y, self.z + vec.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_screen_at() {
        let p1 = Point2D::screen_at(0, 0);
        assert_eq!([0.5, 0.5], [p1.x, p1.y]);

        let p2 = Point2D::screen_at(-1, -1);
        assert_eq!([-0.5, -0.5], [p2.x, p2.y]);
    }

    #[test]
    fn test_point2d_convert() {
        let canvas = Canvas::new(50.0, 50.0, 100.0, 100.0, 100.0);
        let s1 = Point2D::screen_at(0, 0);
        let p1 = s1.convert(&canvas);
        assert_eq!([0.5, 0.5, 100.0], [p1.x, p1.y, p1.z]);

        let s2 = Point2D::screen_at(99, 0);
        let p2 = s2.convert(&canvas);
        assert_eq!([99.5, 0.5, 100.0], [p2.x, p2.y, p2.z]);

        let s3 = Point2D::screen_at(0, 99);
        let p3 = s3.convert(&canvas);
        assert_eq!([0.5, 99.5, 100.0], [p3.x, p3.y, p3.z]);

        let s4 = Point2D::screen_at(99, 99);
        let p4 = s4.convert(&canvas);
        assert_eq!([99.5, 99.5, 100.0], [p4.x, p4.y, p4.z]);
    }

    #[test]
    fn test_point3d_to() {
        let p1 = Point3D::new(5.0, 5.0, 5.0);
        let p2 = Point3D::new(5.0, 0.0, 0.0);
        let v = p1.to(&p2);
        assert_eq!([0.0, -5.0, -5.0], [v.x, v.y, v.z]);
    }

    #[test]
    fn test_point3d_base() {
        let p1 = Point3D::new(5.0, 5.0, 5.0);
        let v1 = Vector3D::new(0.0, -5.0, -3.0);
        let r1 = p1.base(&v1);
        assert_eq!([5.0, 0.0, 2.0], [r1.x, r1.y, r1.z]);
    }
}
