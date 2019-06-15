use std::sync::{Arc, RwLock};
use std::ops::Drop;
use std::iter::Iterator;

use crate::shape::Shape;
use crate::vector::Vector;

use serde::{Deserialize, Serialize, Deserializer, Serializer};

mod collision;
pub use collision::*;

struct ShapeIter<'a> {
    world: &'a World,
    current: usize,
    end: usize,
}
impl<'a> Iterator for ShapeIter<'a> {
    type Item = &'a Shape;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current += 1;
        if current >= self.end {
            return None;
        }
        return Some(&self.world.shapes[current]);
    }
}
impl<'a> ShapeIter<'a> {
    fn new(world: &'a World) -> ShapeIter<'a> {
        ShapeIter{current: 0, end: world.ids_list.len(), world: world}
    }
    fn new_skip(world: &'a World, num: usize) -> ShapeIter<'a> {
        ShapeIter{current: num, end: world.ids_list.len(), world: world}
    }
}

#[derive(Serialize, Deserialize)]
struct DumbWorld (Vec<Shape>);
impl DumbWorld {
    fn to_world(self) -> World {
        let len = self.0.len();
        let mut ids = Vec::with_capacity(len);
        for i in 0..self.0.len() {
            ids.push(i);
        }
        let mut emptycol = Vec::with_capacity(len);
        for _ in 0..len {
            emptycol.push(Arc::new(Vec::new()));
        }
        let DumbWorld(list) = self;
        World{id_counter: len, ids_list: ids, shapes: list, collisions: emptycol}
    }
}

struct World {
    id_counter: usize,
    ids_list: Vec<usize>,
    shapes: Vec<Shape>,
    collisions: Vec<Arc<Vec<Collision>>>,
}
impl World {
    fn new() -> World {
        World{id_counter: 0, ids_list: Vec::new(), shapes: Vec::new(), collisions: Vec::new()}
    }
    fn add_shape(&mut self, shape: Shape) -> usize {
        self.ids_list.push(self.id_counter);
        self.shapes.push(shape);
        self.collisions.push(Arc::new(Vec::new()));
        self.id_counter += 1;
        self.id_counter - 1
    }
    fn remove_shape(&mut self, id: usize) {
        let index = self.ids_list.binary_search(&id).unwrap();
        self.ids_list.remove(index);
        self.shapes.remove(index);
        self.collisions.remove(index);
    }
    fn run(&mut self) {
        let mut results: Vec<Vec<Collision>> = Vec::with_capacity(self.shapes.len());
        for _i in 0..self.shapes.len() {
            results.push(Vec::new());
        }
        for (i, shape) in ShapeIter::new(self).enumerate() {
            for (j, other) in ShapeIter::new_skip(self, i).enumerate() {
                if let Some(res) = shape.resolve(&other) {
                    results[i].push(Collision{
                        other: j,
                        resolution: res,
                    });
                    results[j].push(Collision{
                        other: i,
                        resolution: res * -1.0,
                    })
                }
            }
        }
        let mut new_res: Vec<Arc<Vec<Collision>>> = Vec::with_capacity(results.len());
        for collisions in results.iter() {
            new_res.push(Arc::new(collisions.clone()));
        }
        self.collisions = new_res;
    }
    fn get_shape_mut(&mut self, id: usize) -> &mut Shape {
        let index = self.ids_list.binary_search(&id).unwrap();
        &mut self.shapes[index]
    }
    fn get_shape(&self, id: usize) -> &Shape {
        let index = self.ids_list.binary_search(&id).unwrap();
        &self.shapes[index]
    }
}

pub struct WorldHandle {
    world: Arc<RwLock<World>>,
}
impl WorldHandle {
    pub fn new() -> WorldHandle {
        WorldHandle{world: Arc::new(RwLock::new(World::new()))}
    }
    pub fn add_shape(&self, points: Vec<(f32, f32)>, center: (f32, f32)) -> ShapeHandle {
        let id = self.world.write().unwrap().add_shape(Shape::from_tuples(points, center));
        ShapeHandle{parent: self.world.clone(), id: id}
    }
    pub fn run(&self) {
        self.world.write().unwrap().run()
    }
}
impl Serialize for WorldHandle {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        DumbWorld(self.world.read().unwrap().shapes.clone()).serialize(serializer)
    }
}

// a named tuple with a worldhandle and handles for all of the shapes contained
pub struct WorldShapeHandles(pub WorldHandle, pub Vec<ShapeHandle>);
impl<'de> Deserialize<'de> for WorldShapeHandles {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let world = Arc::new(RwLock::new(DumbWorld::deserialize(deserializer)?.to_world()));
        let mut handles;
        {
            let readworld = world.read().unwrap();
            handles = Vec::with_capacity(readworld.id_counter);
            for i in 0..handles.len() {
                handles.push(ShapeHandle{parent: world.clone(), id: i});
            }
        }
        Ok(WorldShapeHandles(WorldHandle{ world: world }, handles))
    }
}

pub struct ShapeHandle {
    parent: Arc<RwLock<World>>,
    id: usize,
}
impl ShapeHandle {
    pub fn move_by(&mut self, amount: Vector) {
        self.parent.write().unwrap().get_shape_mut(self.id).move_by(amount)
    }
    pub fn rotate(&mut self, angle: f32) {
        self.parent.write().unwrap().get_shape_mut(self.id).rotate(angle)
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn get_collisions(&self) -> Arc<Vec<Collision>> {
        let parent = self.parent.read().unwrap();
        let index = parent.ids_list.binary_search(&self.id).unwrap();
        parent.collisions[index].clone()
    }
    pub fn center(&self) -> Vector {
        self.parent.read().unwrap().get_shape(self.id).center()
    }
}
impl Drop for ShapeHandle {
    fn drop(&mut self) {
        self.parent.write().unwrap().remove_shape(self.id)
    }
}