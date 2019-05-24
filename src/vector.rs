use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector{x: x, y: y}
    }

    pub fn angle(self) -> f32 {
        (self.y/self.x).atan()
    }

    pub fn magnitude(self) -> f32 {
        (self.y.powi(2) + self.x.powi(2)).powf(0.5)
    }

    pub const fn from_tuple((x, y): (f32, f32)) -> Self {
        Vector{x: x, y: y}
    }

    pub fn to_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}