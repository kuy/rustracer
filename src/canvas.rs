use crate::coordinate as coord;
use crate::dim::Dim2D;
use crate::line::Line;
use crate::plane::Plane;
use crate::point::{Point2D, Point3D};
use crate::vector::Vector3D;

pub struct Canvas {
    pub plane: Plane,
    pub dim: Dim2D,
}

impl Canvas {
    pub fn new(x: f32, y: f32, z: f32, w: f32, h: f32) -> Canvas {
        let origin = Point3D { x, y, z };
        let dir = Vector3D::from(x, y, z);
        let dim = Dim2D {
            width: w,
            height: h,
        };
        Canvas {
            plane: Plane { origin, dir },
            dim,
        }
    }

    pub fn cast_ray(&self, pos: &Point2D<coord::Screen>, camera: &Point3D) -> Line {
        let target = pos.convert(self);
        Line {
            origin: camera.clone(),
            dir: camera.to(&target).normalize(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_ray() {
        let canvas = Canvas::new(50.0, 50.0, 50.0, 50.0, 50.0);
        let camera = Point3D::new(50.0, 50.0, 100.0);

        let p1 = Point2D::screen_at(0, 0);
        let r1 = canvas.cast_ray(&p1, &camera);
        assert_eq!(camera, r1.origin);
        assert_eq!(
            [-0.4027503, -0.4027503, -0.8219394],
            [r1.dir.x, r1.dir.y, r1.dir.z]
        );
    }
}
