use std::cmp;
use std::f32::consts::PI;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, AddAssign, Deref, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub const fn new(x: f32, y: f32) -> Vector {
        Vector { x: x, y: y }
    }

    #[inline]
    pub fn from_mag_dir(mag: f32, dir: f32) -> Self {
        Self::new(mag * dir.cos(), mag * dir.sin())
    }

    #[inline]
    pub fn angle(self) -> f32 {
        if self.x >= 0.0 {
            return (self.y / self.x).atan();
        }
        if self.y >= 0.0 {
            return (self.y / self.x).atan() + PI;
        }
        return (self.y / self.x).atan() - PI;
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self.y.powi(2) + self.x.powi(2)).sqrt()
    }

    pub const fn from_tuple((x, y): (f32, f32)) -> Self {
        Vector { x: x, y: y }
    }

    #[inline]
    pub const fn to_tuple(self) -> (f32, f32) {
        (self.x, self.y)
    }

    #[inline]
    pub fn mag_dir(self) -> (f32, f32) {
        (self.magnitude(), self.angle())
    }

    #[inline]
    fn rotated(self, angle: f32) -> Self {
        Self::from_mag_dir(self.magnitude(), self.angle() + angle)
    }

    #[inline]
    pub fn rotated_around(self, around: Self, angle: f32) -> Self {
        (self - around).rotated(angle) + around
    }

    #[inline]
    pub fn abs(self) -> Vector {
        Vector {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Add for Vector {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vector {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl Sub for Vector {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vector {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    #[inline]
    fn mul(self, other: f32) -> Self {
        Vector {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;
    fn div(self, other: f32) -> Self {
        Vector {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Display for Vector {
    fn fmt(&self, form: &mut Formatter) -> Result<(), Error> {
        write!(form, "<{}, {}>", self.x, self.y)
    }
}

pub trait MassConvert {
    fn to_vectors(&self) -> Vec<Vector>;
}

impl<T> MassConvert for T
where
    T: Deref<Target = [(f32, f32)]>,
{
    fn to_vectors(&self) -> Vec<Vector> {
        self
            .iter()
            .map(|&(x, y)| Vector { x: x, y: y })
            .collect()
    }
}

impl MassConvert for [(f32, f32)] {
    fn to_vectors(&self) -> Vec<Vector> {
        self.iter().map(|&(x, y)| Vector { x: x, y: y }).collect()
    }
}

impl cmp::PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl cmp::PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}
