use crate::shape::Shape;
use crate::lines::*;
use std::marker::PhantomData;
use amethyst_core::math::{Point2, Vector2};

pub mod system;

pub trait Process<'a, T, I>: Sized {
    fn raycast(self, start: Point2<f32>, angle: f32) -> Raycast<'a, I, T>;
}

impl<'a, H: Iterator<Item=S>, S: Split<'a, T>, T: 'a, I: IntoIterator<Item=S, IntoIter=H> + Sized> Process<'a, T, H> for I {
    fn raycast(self, start: Point2<f32>, angle: f32) -> Raycast<'a, H, T> {
        Raycast::new(self.into_iter(), start, angle)
    }
}

pub trait Split<'a, T> {
    fn split(self) -> (&'a Shape, &'a T);
}

impl<'a, T> Split<'a, T> for &'a (Shape, T) {
    fn split(self) -> (&'a Shape, &'a T) {
        (&self.0, &self.1)
    }
}

impl<'a, T> Split<'a, T> for (&'a Shape, &'a T) {
    fn split(self) -> (&'a Shape, &'a T) {
        self
    }
}


#[derive(Debug, PartialEq)]
pub struct RayCollision<T> {
    pub tag: T,
    pub dist: Vector2<f32>,
}

pub struct Raycast<'a, I, T> {
    ray: Line,
    normal: InEq,
    iter: I,
    ghost: PhantomData<&'a T>,
}
impl<'a, T: 'a, S: Split<'a, T>, I: Iterator<Item=S>> Iterator for Raycast<'a, I, T> {
    type Item = RayCollision<&'a T>;

    fn next(&mut self) -> Option<RayCollision<&'a T>> {
        while let Some(spl) = self.iter.next() {
            let (shape, tag) = spl.split();
            if let Some(point) = shape.receive_ray(self.ray, self.normal) {
                return Some(RayCollision {
                    tag,
                    dist: point,
                })
            }
        }
        None
    }
}
impl<'a, T: 'a, S: Split<'a, T>, I: Iterator<Item=S>> Raycast<'a, I, T>  {
    pub fn new<D: IntoIterator<IntoIter=I, Item=S>>(into_iterator: D, start: Point2<f32>, angle: f32) -> Self {
        let calibrator = Point2::from(comp_vec(1.0, angle) + Vector2::new(start.x, start.y));
        let ray = Line::through(start, calibrator);
        let normal = ray.normal_through(start).initialize(calibrator);
        Raycast {
            ray,
            normal,
            iter: into_iterator.into_iter(),
            ghost: PhantomData
        }
    }
}

fn comp_vec(mag: f32, dir: f32) -> Vector2<f32> {
    Vector2::new(mag * dir.cos(), mag * dir.sin())
}