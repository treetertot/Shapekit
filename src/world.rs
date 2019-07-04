use crate::shape::Shape;
use crate::vector::Vector;
pub mod collision;
use collision::Collision;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::sync::{Arc, RwLock};
use std::ops::Drop;
use std::iter::Iterator;

#[derive(Serialize, Deserialize)]
struct World<Tag> {
    id_counter: usize,
    shapes: Vec<(usize, Shape, Tag)>,
}
impl<T: Clone> World<T> {
    fn get_shape(&self, id: usize) -> &Shape {
        &self.shapes[self.shapes.binary_search_by(|(probe, _, _)| probe.cmp(&id)).unwrap()].1
    }
    fn get_shape_mut(&mut self, id: usize) -> &mut Shape {
        let index = self.shapes.binary_search_by(|(probe, _, _)| probe.cmp(&id)).unwrap();
        &mut self.shapes[index].1
    }
    fn remove_shape(&mut self, id: usize) {
        let index = self.shapes.binary_search_by(|(probe, _, _)| probe.cmp(&id)).unwrap();
        self.shapes.remove(index);
    }
    fn add_shape(&mut self, shape: Shape, tag: T) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        self.shapes.push((id, shape, tag));
        id
    }
}

pub struct WorldHandle<T: Clone>(Arc<RwLock<World<T>>>);
impl<T: Clone> WorldHandle<T> {
    pub fn add_shape(&self, points: &Vec<(f32, f32)>, start: (f32, f32), tag: T) -> ShapeHandle<T> {
        ShapeHandle{ id: self.0.write().unwrap().add_shape(Shape::from_tuples(points, start), tag), world: self.0.clone() }
    }
    pub fn add_in_place(&self, points: &Vec<(f32, f32)>, tag: T) -> ShapeHandle<T> {
        ShapeHandle{ id: self.0.write().unwrap().add_shape(Shape::in_place_tuples(points), tag), world: self.0.clone() }
    }
    pub fn new() -> Self {
        WorldHandle(Arc::new(RwLock::new(World{id_counter: 0, shapes: Vec::new()})))
    }
}
impl<T: Clone> Serialize for WorldHandle<T> where T: Serialize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.read().unwrap().serialize(serializer)
    }
}

pub struct WorldAndShapeHandles<T: Clone>(WorldHandle<T>, Vec<ShapeHandle<T>>);
impl<'de, T: Clone> Deserialize<'de> for WorldAndShapeHandles<T> where T: Deserialize<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let world = Arc::new(RwLock::new(World::deserialize(deserializer)?));
        let mut handles = Vec::new();
        for (id, _, _) in world.read().unwrap().shapes.iter() {
            handles.push(ShapeHandle{ world: world.clone(), id: *id })
        }
        Ok(WorldAndShapeHandles(WorldHandle(world), handles))
    }
}

pub struct ShapeHandle<T: Clone> {
    world: Arc<RwLock<World<T>>>,
    id: usize,
}
impl<T: Clone> ShapeHandle<T> {
    pub fn move_by(&mut self, v: Vector) {
        let mut world = self.world.write().unwrap();
        world.get_shape_mut(self.id).move_by(v);
    }
    pub fn center(&self) -> Vector {
        let world = self.world.read().unwrap();
        world.get_shape(self.id).center()
    }
    pub fn bottom_left(&self) -> Vector {
        let world = self.world.read().unwrap();
        world.get_shape(self.id).center()
    }
    pub fn collisions(&self) -> CollisionIter<T> {
        let world = self.world.read().unwrap();
        let shape_a = world.get_shape(self.id);
        let mut list = Vec::new();
        for (i, shape_b, info) in world.shapes.iter() {
            if *i == self.id {
                continue;
            }
            if let Some(res) = shape_a.resolve(shape_b) {
                list.push(Collision{ other: info.clone(), resolution: res });
            }
        }
        CollisionIter { list: list }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
}
impl<T: Clone> Drop for ShapeHandle<T> {
    fn drop(&mut self) {
        self.world.write().unwrap().remove_shape(self.id);
    }
}

pub struct CollisionIter<T: Clone> {
    list: Vec<Collision<T>>
}
impl<T: Clone> Iterator for CollisionIter<T> {
    type Item = Collision<T>;
    #[inline]
    fn next(&mut self) -> Option<Collision<T>> {
        self.list.pop()
    }
}

pub fn compare<T: Clone>(out: &mut Vec<(usize, Collision<T>)>, (id_a, shapea, taga): &(usize, Shape, T), (id_b, shapeb, tagb): &(usize, Shape, T)) {
    if let Some(res) = shapea.resolve(shapeb) {
        let index = match out.binary_search_by(|(probe, _)| probe.cmp(id_a)) {
            Ok(val) => val,
            Err(val) => val,
        };
        out.insert(index, (*id_a, Collision{ other: tagb.clone(), resolution: res}));
        let index = match out.binary_search_by(|(probe, _)| probe.cmp(id_b)) {
            Ok(val) => val,
            Err(val) => val,
        };
        out.insert(index, (*id_b, Collision{ other: taga.clone(), resolution: res * -1.0}));
    }
}