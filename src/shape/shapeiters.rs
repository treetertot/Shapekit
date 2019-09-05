use crate::lines::{InEq, Line};
use crate::vector::Vector;
use std::iter::Peekable;
use std::slice::Iter;
pub struct PointsIter<'a> {
    pub points: Iter<'a, Vector>,
    pub displacement: Vector,
}
impl<'a> Iterator for PointsIter<'a> {
    type Item = Vector;
    fn next(&mut self) -> Option<Vector> {
        Some(*self.points.next()? + self.displacement)
    }
}
pub struct SidesIter<'a> {
    pub points: Peekable<PointsIter<'a>>,
    pub center: Vector,
    pub first: Vector,
}
impl<'a> Iterator for SidesIter<'a> {
    type Item = InEq;
    fn next(&mut self) -> Option<InEq> {
        let a = self.points.next()?;
        match self.points.peek() {
            Some(&b) => Some(Line::through(a, b).initialize(self.center)),
            None => Some(Line::through(a, self.first).initialize(self.center)),
        }
    }
}
