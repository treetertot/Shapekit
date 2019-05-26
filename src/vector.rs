use std::ops::{Add, Sub, Mul, AddAssign, SubAssign};

use std::fmt::{Display, Formatter, Error};

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

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Vector{x: self.x * rhs, y: self.y * rhs}
    }
}

impl Display for Vector {
    fn fmt(&self, form: &mut Formatter) -> Result<(), Error> {
        write!(form, "<{}, {}>", self.x, self.y)
    }
}