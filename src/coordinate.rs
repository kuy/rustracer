pub trait Coordinate {}

#[derive(Debug)]
pub struct Screen();
impl Coordinate for Screen {}

#[derive(Debug)]
pub struct Canvas();
impl Coordinate for Canvas {}

#[derive(Debug)]
pub struct World();
impl Coordinate for World {}
