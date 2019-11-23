use crate::line::Line;
use crate::point::Point3D;
use crate::vector::{General, Normal, Vector3D};

pub struct Sphere {
    pub center: Point3D,
    pub radius: f32,
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Sphere {
        Sphere {
            center: Point3D::new(x, y, z),
            radius: r,
        }
    }

    pub fn intersection(&self, line: &Line) -> Option<Vec<Point3D>> {
        let v1 = line.origin.to(&self.center);
        assert_eq!(5.0, v1.norm());
        assert!(false);

        let dp = line.dir.dot(&v1);

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let s = Sphere::new(10.0, 0.0, 10.0, 5.0);
        let l1 = Line {
            origin: Point3D::new(7.0, 0.0, 20.0),
            dir: Vector3D::<Normal>::from(0.0, 0.0, -1.0),
        };

        let i1 = s.intersection(&l1);
        assert!(i1.is_some());

        let i1 = i1.unwrap();
        assert_eq!(2, i1.len());
        assert_eq!([7.0, 0.0, 6.0], [i1[0].x, i1[0].y, i1[0].z]);
        assert_eq!([7.0, 0.0, 14.0], [i1[1].x, i1[1].y, i1[1].z]);
    }
}
