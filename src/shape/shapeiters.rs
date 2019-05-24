use crate::lines::InEQ;
use crate::shape::Shape;
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