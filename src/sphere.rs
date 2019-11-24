use crate::line::Line;
use crate::point::Point3D;

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
        let nv1 = v1.norm();
        let dp = line.dir.dot(&v1);

        let v2 = line.dir.mul(dp);
        let nv2 = v2.norm();

        let a = (nv1.powi(2) - nv2.powi(2)).sqrt();
        if a > self.radius {
            None
        } else {
            let b = (self.radius.powi(2) - a.powi(2)).sqrt();

            let ov1 = line.dir.mul(nv2 - b);
            let p1 = line.origin.base(&ov1);

            // TODO: Naive impl.
            if a == self.radius {
                Some(vec![p1])
            } else {
                let ov2 = line.dir.mul(nv2 + b);
                let p2 = line.origin.base(&ov2);

                Some(vec![p1, p2])
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::{Normal, Vector3D};

    #[test]
    fn test_intersection() {
        let s = Sphere::new(10.0, 0.0, 10.0, 5.0);
        let l1 = Line {
            origin: Point3D::new(7.0, 0.0, 20.0),
            dir: Vector3D::from(0.0, 0.0, -1.0),
        };

        let i1 = s.intersection(&l1);
        assert!(i1.is_some());

        let i1 = i1.unwrap();
        assert_eq!(2, i1.len());
        assert_eq!([7.0, 0.0, 14.0], [i1[0].x, i1[0].y, i1[0].z]);
        assert_eq!([7.0, 0.0, 6.0], [i1[1].x, i1[1].y, i1[1].z]);
    }

    #[test]
    fn test_intersection_center() {
        let s = Sphere::new(10.0, 0.0, 10.0, 5.0);
        let l1 = Line {
            origin: Point3D::new(10.0, 0.0, 20.0),
            dir: Vector3D::from(0.0, 0.0, -1.0),
        };

        let i1 = s.intersection(&l1);
        assert!(i1.is_some());

        let i1 = i1.unwrap();
        assert_eq!(2, i1.len());
        assert_eq!([10.0, 0.0, 15.0], [i1[0].x, i1[0].y, i1[0].z]);
        assert_eq!([10.0, 0.0, 5.0], [i1[1].x, i1[1].y, i1[1].z]);
    }

    #[test]
    fn test_intersection_zero() {
        let s = Sphere::new(10.0, 0.0, 10.0, 5.0);
        let l1 = Line {
            origin: Point3D::new(20.0, 0.0, 20.0),
            dir: Vector3D::from(0.0, 0.0, -1.0),
        };

        let i1 = s.intersection(&l1);
        assert!(i1.is_none());
    }

    #[test]
    fn test_intersection_one() {
        let s = Sphere::new(10.0, 0.0, 10.0, 5.0);
        let l1 = Line {
            origin: Point3D::new(15.0, 0.0, 20.0),
            dir: Vector3D::from(0.0, 0.0, -1.0),
        };

        let i1 = s.intersection(&l1);
        assert!(i1.is_some());

        let i1 = i1.unwrap();
        assert_eq!(1, i1.len());
        assert_eq!([15.0, 0.0, 10.0], [i1[0].x, i1[0].y, i1[0].z]);
    }
}
