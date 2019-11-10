use crate::point::Point3D;
use crate::vector::{Normal, Vector3D};

pub struct Plane {
    pub origin: Point3D,
    pub dir: Vector3D<Normal>,
}
