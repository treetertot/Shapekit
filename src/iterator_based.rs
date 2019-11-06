use crate::vector::Vector;
use crate::shape::Shape;
use std::mem;

pub struct Collision<T> {
    pub collider: T,
    pub resolution: Vector,
    pub other: T,
}

pub struct Collisions<'a, I, T> {
    iter: I,
    index: usize,
    collected: Vec<(&'a Shape, &'a T)>,
    grab: Option<Collision<&'a T>>,
}

impl<'a, T, I: Iterator<Item=(&'a Shape, &'a T)>> Collisions<'a, I, T> {
    pub fn new<D: IntoIterator<IntoIter=I, Item=(&'a Shape, &'a T)>>(into_iterator: D) -> Self {
        let iter = into_iterator.into_iter();
        Collisions {
            collected: Vec::with_capacity(iter.size_hint().0),
            iter,
            index: 0,
            grab: None,
        }
    }
}

impl<'a, T, I: Iterator<Item=(&'a Shape, &'a T)>> Iterator for Collisions<'a, I, T> {
    type Item = Collision<&'a T>;

    fn next(&mut self) -> Option<Collision<&'a T>> {
        if self.grab.is_some() {
            return mem::replace(&mut self.grab, None);
        }
        loop {
            while self.index < self.collected.len() {
                if self.index == self.collected.len()-1 {
                    break;
                }
                let &(c_shape, c_tag) = self.collected.last()?;
                let &(o_shape, o_tag) = self.collected.last()?;
                if let Some(resolution) = c_shape.resolve(o_shape) {
                    self.grab = Some(Collision {
                        collider: o_tag,
                        resolution: resolution * -1.,
                        other: c_tag,
                    });
                    return Some(Collision {
                        collider: c_tag,
                        resolution,
                        other: o_tag,
                    });
                }
                self.index += 1;
            }
            self.collected.push(self.iter.next()?);
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let min = if self.grab.is_some() {
            1
        } else {
            0
        };
        (min, self.iter.size_hint().1)
    }
}
