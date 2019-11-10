use std::marker::PhantomData;

pub trait Kind {}

#[derive(Debug)]
pub struct General();
impl Kind for General {}

#[derive(Debug)]
pub struct Normal();
impl Kind for Normal {}

#[derive(Debug)]
pub struct Vector3D<T: Kind> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _t: PhantomData<T>,
}

impl<T: Kind> Vector3D<T> {
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vector3D<General> {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3D<General> {
        Vector3D::<General> {
            x,
            y,
            z,
            _t: PhantomData,
        }
    }

    pub fn normalize(&self) -> Vector3D<Normal> {
        Vector3D::<Normal>::from(self.x, self.y, self.z)
    }
}

impl Vector3D<Normal> {
    pub fn from(x: f32, y: f32, z: f32) -> Vector3D<Normal> {
        let len = (x.powi(2) + y.powi(2) + z.powi(2)).sqrt();
        Vector3D::<Normal> {
            x: x / len,
            y: y / len,
            z: z / len,
            _t: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3d_length() {
        let v = Vector3D::<General>::new(3.0, 4.0, 5.0);
        assert_eq!(7.071068, v.length());
    }

    #[test]
    fn test_vector3d_dot() {
        let v1 = Vector3D::<General>::new(5.0, 4.0, 1.0);
        assert_eq!(42.0, v1.dot(&v1));
        assert_eq!(v1.dot(&v1).sqrt(), v1.length());

        let v2 = Vector3D::<General>::new(2.0, 3.0, 5.0);
        assert_eq!(27.0, v1.dot(&v2));

        let v3 = Vector3D::<General>::new(5.0, 5.0, 0.0);
        let v4 = Vector3D::<General>::new(0.0, 0.0, 5.0);
        assert_eq!(0.0, v3.dot(&v4));
    }

    #[test]
    fn test_vector3d_norm() {
        let v = Vector3D::<General>::new(3.0, 4.0, 5.0);
        let n = v.normalize();
        assert_eq!([0.42426407, 0.56568545, 0.70710677], [n.x, n.y, n.z]);
    }

    #[test]
    fn test_vector3d_from() {
        let n0 = Vector3D::<Normal>::from(3.0, 4.0, 5.0);
        let v = Vector3D::<General>::new(3.0, 4.0, 5.0);
        let n = v.normalize();
        assert_eq!([n0.x, n0.y, n0.z], [n.x, n.y, n.z]);
    }
}
