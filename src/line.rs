use crate::point::Point3D;
use crate::vector::{Normal, Vector3D};

#[derive(Debug)]
pub struct Line {
    pub origin: Point3D,
    pub dir: Vector3D<Normal>,
}
