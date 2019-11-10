use std::marker::PhantomData;

#[derive(Debug)]
pub struct General();

#[derive(Debug)]
pub struct Normal();

#[derive(Debug)]
pub struct Vector3D<T> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _t: PhantomData<T>,
}

impl<T> Vector3D<T> {
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
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

    pub fn norm(&self) -> Vector3D<Normal> {
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
