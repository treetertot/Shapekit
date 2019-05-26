use crate::lines::InEQ;
use crate::shape::Shape;
use crate::vector::Vector;

use std::iter::Iterator;

pub struct IneqIter<'a> {
    shape: &'a Shape,
    counter: usize,
}

impl<'a> IneqIter<'a> {
    pub fn new(shape: &Shape) -> IneqIter {
        IneqIter{shape: shape, counter: 0}
    }
}

impl<'a> Iterator for IneqIter<'a> {
    type Item = InEQ;

    fn next(&mut self) -> Option<InEQ> {
        self.counter += 1;
        if self.counter - 1 < self.shape.points.len() {
            return Some(self.shape.get_ineq(self.counter - 1));
        }
        None
    }
}

pub struct PointsIter<'a> {
    shape: &'a Shape,
    counter: usize,
}

impl<'a> PointsIter<'a> {
    pub fn new(shape: &Shape) -> PointsIter {
        PointsIter{shape: shape, counter: 0}
    }
}

impl<'a> Iterator for PointsIter<'a> {
    type Item = Vector;

    fn next(&mut self) -> Option<Vector> {
        self.counter += 1;
        if self.counter - 1 < self.shape.points.len() {
            return Some(self.shape.get_point(self.counter - 1));
        }
        None
    }
}