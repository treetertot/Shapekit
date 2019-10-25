use crate::shape::Shape;
use crate::vector::Vector;
use std::ops::Drop;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct PhysicsWorld<T: 'static + Clone + Send + Sync>(
    Arc<RwLock<Vec<(usize, T, RwLock<Shape>)>>>,
);
impl<T: 'static + Clone + Send + Sync> PhysicsWorld<T> {
    pub fn new() -> PhysicsWorld<T> {
        PhysicsWorld(Arc::new(RwLock::new(Vec::new())))
    }
    pub fn add_shape(&self, points: Vec<Vector>, tag: T) -> ShapeHandle<T> {
        let mut guard = self.0.write().unwrap();
        let count = match guard.last() {
            Some((val, _, _)) => val + 1,
            None => 0,
        };
        let mut new_points = Vec::with_capacity(points.len());
        for i in 0..points.len() - 1 {
            new_points.push(points[i]);
            new_points.push((points[i] + points[i + 1]) / 2.0);
        }
        match points.last() {
            Some(last) => {
                new_points.push(*last);
                new_points.push((*last + *points.first().unwrap()) / 2.0);
            }
            None => (),
        }
        guard.push((count, tag, RwLock::new(Shape::new(new_points))));
        ShapeHandle {
            world: self.clone(),
            id: count,
        }
    }
    pub fn add_shapes(&self, shapes: Vec<(Vec<Vector>, T)>) -> Vec<ShapeHandle<T>> {
        let mut guard = self.0.write().unwrap();
        let mut count = match guard.last() {
            Some((val, _, _)) => val + 1,
            None => 0,
        };
        let mut out = Vec::with_capacity(shapes.len());
        for (points, tag) in shapes {
            guard.push((count, tag, RwLock::new(Shape::new(points))));
            out.push(ShapeHandle {
                world: self.clone(),
                id: count,
            });
            count += 1;
        }
        out
    }
}

pub struct ShapeHandle<T: 'static + Clone + Send + Sync> {
    world: PhysicsWorld<T>,
    id: usize,
}
impl<T: 'static + Clone + Send + Sync> ShapeHandle<T> {
    pub fn points(&self) -> Vec<Vector> {
        let guard = self.world.0.read().unwrap();
        let index = get_index(&guard, self.id);
        let new_guard = guard[index].2.read().unwrap();
        new_guard.iter_points().collect()
    }
    pub fn tag(&self) -> T {
        let guard = self.world.0.read().unwrap();
        let index = get_index(&guard, self.id);
        guard[index].1.clone()
    }
    pub fn move_by(&self, v: Vector) {
        let guard = self.world.0.read().unwrap();
        let index = get_index(&guard, self.id);
        let mut new_guard = guard[index].2.write().unwrap();
        new_guard.displacement += v;
    }
    pub fn collisions(&self) -> Vec<(Vector, T)> {
        let guard = self.world.0.read().unwrap();
        let index = get_index(&guard, self.id);
        let new_guard = guard[index].2.read().unwrap();
        guard
            .iter()
            .filter(|(id, _, _)| *id != self.id)
            .map(|(_, tag, shapelock)| (new_guard.resolve(&shapelock.read().unwrap()), tag.clone()))
            .filter(|(maybevec, _tag)| maybevec.is_some())
            .map(|(vec, tag)| (vec.unwrap(), tag))
            .collect()
    }
}

impl<T: 'static + Clone + Send + Sync> Drop for ShapeHandle<T> {
    fn drop(&mut self) {
        let mut guard = self.world.0.write().unwrap();
        let index = get_index(&guard, self.id);
        guard.remove(index);
    }
}

fn get_index<T: 'static + Clone + Send + Sync>(
    collection: &[(usize, T, RwLock<Shape>)],
    id: usize,
) -> usize {
    collection
        .binary_search_by_key(&id, |(key, _, _)| *key)
        .unwrap()
}
