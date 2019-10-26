use crate::lines::{InEq, Line};
use crate::vector::Vector;
use std::iter::Peekable;
use std::slice;
pub struct SidesIter<'a> {
    pub points: Peekable<slice::Iter<'a, Vector>>,
    pub center: Vector,
    pub first: Vector,
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
    pub points: Peekable<slice::Iter<'a, Vector>>,
    pub first: Vector,
}
impl<'a> Iterator for Mangled<'a> {
    type Item = (Line, Vector, Vector);
    fn next(&mut self) -> Option<(Line, Vector, Vector)> {
        let a = *self.points.next()?;
        match self.points.peek() {
            Some(&&b) => Some((Line::through(a, b), a, b)),
            None => Some((Line::through(a, self.first), a, self.first)),
        }
    }
}
