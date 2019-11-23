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
    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn add(&self, other: &Self) -> Vector3D<General> {
        Vector3D::<General>::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn mul(&self, k: f32) -> Vector3D<General> {
        Vector3D::<General>::new(self.x * k, self.y * k, self.z * k)
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
    fn test_vector3d_norm() {
        let v = Vector3D::<General>::new(3.0, 4.0, 5.0);
        assert_eq!(7.071068, v.norm());
    }

    #[test]
    fn test_vector3d_dot() {
        let v1 = Vector3D::<General>::new(5.0, 4.0, 1.0);
        assert_eq!(42.0, v1.dot(&v1));
        assert_eq!(v1.dot(&v1).sqrt(), v1.norm());

        let v2 = Vector3D::<General>::new(2.0, 3.0, 5.0);
        assert_eq!(27.0, v1.dot(&v2));

        let v3 = Vector3D::<General>::new(5.0, 5.0, 0.0);
        let v4 = Vector3D::<General>::new(0.0, 0.0, 5.0);
        assert_eq!(0.0, v3.dot(&v4));
    }

    #[test]
    fn test_vector3d_add() {
        let v1 = Vector3D::<General>::new(3.0, 4.0, 5.0);
        let v2 = Vector3D::<General>::new(5.0, 4.0, 3.0);
        let v3 = v1.add(&v2);
        assert_eq!([8.0, 8.0, 8.0], [v3.x, v3.y, v3.z]);
    }

    #[test]
    fn test_vector3d_mul() {
        let v = Vector3D::<General>::new(3.0, 4.0, 5.0);
        let r = v.mul(3.0);
        assert_eq!([9.0, 12.0, 15.0], [r.x, r.y, r.z]);
    }

    #[test]
    fn test_vector3d_normalize() {
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
