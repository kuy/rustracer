use crate::coordinate::Coordinate;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Point2D<T: Coordinate> {
    pub x: f32,
    pub y: f32,
    _c: PhantomData<T>,
}
