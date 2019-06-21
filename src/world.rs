use crate::shape::Shape;
use crate::vector::Vector;
pub mod collision;
use collision::Collision;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use std::sync::{Arc, RwLock};
use std::ops::Drop;
use std::iter::Iterator;

#[derive(Serialize, Deserialize)]
struct World {
    id_counter: usize,
    shapes: Vec<(usize, Shape)>,
}
impl World {
    fn get_shape(&self, id: usize) -> &Shape {
        &self.shapes[self.shapes.binary_search_by(|(probe, _)| probe.cmp(&id)).unwrap()].1
    }
    fn get_shape_mut(&mut self, id: usize) -> &mut Shape {
        let index = self.shapes.binary_search_by(|(probe, _)| probe.cmp(&id)).unwrap();
        &mut self.shapes[index].1
    }
    fn remove_shape(&mut self, id: usize) {
        let index = self.shapes.binary_search_by(|(probe, _)| probe.cmp(&id)).unwrap();
        self.shapes.remove(index);
    }
    fn add_shape(&mut self, shape: Shape) -> usize {
        let id = self.id_counter;
        self.id_counter += 1;
        self.shapes.push((id, shape));
        id
    }
}

pub struct WorldHandle (Arc<RwLock<World>>);
impl WorldHandle {
    pub fn add_shape(&self, points: Vec<(f32, f32)>, start: (f32, f32)) -> ShapeHandle {
        ShapeHandle{ id: self.0.write().unwrap().add_shape(Shape::from_tuples(points, start)), world: self.0.clone() }
    }
    pub fn new() -> Self {
        WorldHandle(Arc::new(RwLock::new(World{id_counter: 0, shapes: Vec::new()})))
    }
}
impl Serialize for WorldHandle {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.read().unwrap().serialize(serializer)
    }
}

pub struct WorldAndShapeHandles (WorldHandle, Vec<ShapeHandle>);
impl<'de> Deserialize<'de> for WorldAndShapeHandles {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let world = Arc::new(RwLock::new(World::deserialize(deserializer)?));
        let mut handles = Vec::new();
        for (id, _) in world.read().unwrap().shapes.iter() {
            handles.push(ShapeHandle{ world: world.clone(), id: *id })
        }
        Ok(WorldAndShapeHandles(WorldHandle(world), handles))
    }
}

pub struct ShapeHandle {
    world: Arc<RwLock<World>>,
    id: usize,
}
impl ShapeHandle {
    pub fn move_by(&mut self, v: Vector) {
        let mut world = self.world.write().unwrap();
        world.get_shape_mut(self.id).move_by(v);
    }
    pub fn rotate(&mut self, r: f32) {
        let mut world = self.world.write().unwrap();
        world.get_shape_mut(self.id).rotate(r);
    }
    pub fn center(&self) -> Vector {
        let world = self.world.read().unwrap();
        world.get_shape(self.id).center()
    }
    pub fn collisions(&self) -> CollisionIter {
        let world = self.world.read().unwrap();
        let shape_a = world.get_shape(self.id);
        let mut list = Vec::new();
        for (i, shape_b) in world.shapes.iter() {
            if *i == self.id {
                continue;
            }
            if let Some(res) = shape_a.resolve(shape_b) {
                list.push(Collision{ other: *i, resolution: res });
            }
        }
        CollisionIter { list: list }
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
}
impl Drop for ShapeHandle {
    fn drop(&mut self) {
        self.world.write().unwrap().remove_shape(self.id);
    }
}

pub struct CollisionIter {
    list: Vec<Collision>
}
impl Iterator for CollisionIter {
    type Item = Collision;
    #[inline]
    fn next(&mut self) -> Option<Collision> {
        self.list.pop()
    }
}

pub fn compare(out: &mut Vec<(usize, Collision)>, shapea: &Shape, shapeb: &Shape, id_a: usize, id_b: usize) {
    if let Some(res) = shapea.resolve(shapeb) {
        let index = match out.binary_search_by(|(probe, _)| probe.cmp(&id_a)) {
            Ok(val) => val,
            Err(val) => val,
        };
        out.insert(index, (id_a, Collision{ other: id_b, resolution: res}));
        let index = match out.binary_search_by(|(probe, _)| probe.cmp(&id_b)) {
            Ok(val) => val,
            Err(val) => val,
        };
        out.insert(index, (id_b, Collision{ other: id_a, resolution: res * -1.0}));
    }
}