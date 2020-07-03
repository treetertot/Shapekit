use crate::lines::{InEq, Line};
use std::iter::Peekable;
use std::slice;
use amethyst_core::math::Point2;

pub struct SidesIter<'a> {
    pub points: Peekable<slice::Iter<'a, Point2<f32>>>,
    pub center: Point2<f32>,
    pub first: Point2<f32>
}
impl<'a> Iterator for SidesIter<'a> {
    type Item = InEq;
    fn next(&mut self) -> Option<InEq> {
        let a = *self.points.next()?;
        match self.points.peek() {
            Some(&&b) => Some(Line::through(a, b).initialize(self.center)),
            None => Some(Line::through(a, self.first).initialize(self.center)),
        }
    }
}
impl<'a> SidesIter<'a> {
    pub fn mangle(self) -> Mangled<'a> {
        Mangled {
            points: self.points,
            first: self.first,
        }
    }
}

pub struct Mangled<'a> {
    pub points: Peekable<slice::Iter<'a, Point2<f32>>>,
    pub first: Point2<f32>
}
impl<'a> Iterator for Mangled<'a> {
    type Item = (Line, Point2<f32>, Point2<f32>);

    fn next(&mut self) -> Option<Self::Item> {
        let a = *self.points.next()?;
        match self.points.peek() {
            Some(&&b) => Some((Line::through(a, b), a, b)),
            None => Some((Line::through(a, self.first), a, self.first)),
        }
    }
}
