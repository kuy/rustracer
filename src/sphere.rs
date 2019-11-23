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
        // s: vector from center of sphere to origin of line
        let s = Vector3D::<General>::new(
            line.origin.x - self.center.x,
            line.origin.y - self.center.y,
            line.origin.z - self.center.z,
        );
        let sn = s.normalize();

        let a = line.dir.norm().powi(2);
        let b = 2.0 * sn.dot(&line.dir);
        let c = s.norm().powi(2) - self.radius.powi(2);

        let d = b.powi(2) - 4.0 * a * c;
        println!("{}", d);
        if d < 0.0 {
            return None;
        }

        if d == 0.0 {
            let t = -b / (2.0 * a);
            let p = s.add(&line.dir.mul(t));
            Some(vec![Point3D::new(
                self.center.x + p.x,
                self.center.y + p.y,
                self.center.z + p.z,
            )])
        } else {
            let tp = (-b + d.sqrt()) / (2.0 * a);
            let tn = (-b - d.sqrt()) / (2.0 * a);
            let pp = s.add(&line.dir.mul(tp));
            let pn = s.add(&line.dir.mul(tn));
            Some(vec![
                Point3D::new(
                    self.center.x + pp.x,
                    self.center.y + pp.y,
                    self.center.z + pp.z,
                ),
                Point3D::new(
                    self.center.x + pn.x,
                    self.center.y + pn.y,
                    self.center.z + pn.z,
                ),
            ])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let s = Sphere::new(0.0, 0.0, 0.0, 10.0);
        let l1 = Line {
            origin: Point3D::new(0.0, 0.0, 50.0),
            dir: Vector3D::<Normal>::from(0.0, 0.0, -1.0),
        };

        let i1 = s.intersection(&l1);
        assert!(i1.is_some());

        let ps1 = i1.unwrap();
        assert_eq!(2, ps1.len());
        assert_eq!([0.0, 0.0, 10.0], [ps1[0].x, ps1[0].y, ps1[0].z]);
        assert_eq!([1.0, 0.0, -10.0], [ps1[1].x, ps1[1].y, ps1[1].z]);
    }
}
