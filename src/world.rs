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
    collisions: Vec<(usize, Collision)>,
    move_tags: Vec<usize>,
}
impl World {
    fn check(&mut self) {
        let mut new_colls = Vec::new();
        for &current_moved in self.move_tags.iter() {
            let mut passed = true;
            for (id_b, shape_b) in self.shapes.iter() {
                if current_moved == *id_b {
                    passed = true;
                    println!("check evaded");
                    continue;
                }
                if passed {
                    self.check_two(&mut new_colls, current_moved, *id_b, shape_b);
                    println!("WHYYYYYYYYYYYYYYYYYYYY");
                } else {
                    match self.move_tags.binary_search(&id_b) {
                        Ok(_) => continue,
                        Err(_) => (),
                    }
                    self.check_two(&mut new_colls, current_moved, *id_b, shape_b);
                }
            }
        }
        self.collisions = new_colls;
        self.move_tags = Vec::with_capacity(self.move_tags.len());
    }
    fn check_two(&self, out: &mut Vec<(usize, Collision)>, id_a: usize, id_b: usize, shape_b: &Shape) {
        let index_a = self.shapes.binary_search_by(|(probe, _)| probe.cmp(&id_a)).unwrap();
        let shape_a = &self.shapes[index_a].1;
        if let Some(res) = shape_a.resolve(shape_b) {
            let coll_index = match self.collisions.binary_search_by(|(probe, _)| probe.cmp(&id_a)) {
                Ok(val) => val,
                Err(val) => val,
            };
            out.insert(coll_index, (id_a, Collision{other: id_b, resolution: res}));
            let coll_index = match self.collisions.binary_search_by(|(probe, _)| probe.cmp(&id_b)) {
                Ok(val) => val,
                Err(val) => val,
            };
            out.insert(coll_index, (id_b, Collision{other: id_a, resolution: res * -1.0}));
        }
    }
    fn get_collision(&mut self, id: usize) -> Option<Collision> {
        if self.collisions.len() == 0 {
            self.check();
        }
        match self.collisions.binary_search_by(|(probe, _)| probe.cmp(&id)) {
            Ok(index) => {
                Some(self.collisions.remove(index).1)
            },
            Err(_) => None,
        }
    }
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
        match self.move_tags.binary_search(&id) {
            Ok(_) => panic!("tried to add shape that aldready exists"),
            Err(index) => self.move_tags.insert(index, id),
        }
        match self.shapes.binary_search_by(|(probe, _)| probe.cmp(&id)) {
            Ok(_) => panic!("tried to add shape that aldready exists"),
            Err(index) => self.shapes.insert(index, (id, shape)),
        }
        id
    }
}

pub struct WorldHandle (Arc<RwLock<World>>);
impl WorldHandle {
    pub fn add_shape(&self, points: Vec<(f32, f32)>, start: (f32, f32)) -> ShapeHandle {
        ShapeHandle{ id: self.0.write().unwrap().add_shape(Shape::from_tuples(points, start)), world: self.0.clone()}
    }
    pub fn new() -> Self {
        WorldHandle(Arc::new(RwLock::new(World{id_counter: 0, shapes: Vec::new(), collisions: Vec::new(), move_tags: Vec::new()})))
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
    fn get_collision(&self) -> Option<Collision> {
        self.world.write().unwrap().get_collision(self.id)
    }
    pub fn collisions<'a>(&'a self) -> CollisionIter<'a> {
        CollisionIter { handle: self }
    }
}
impl Drop for ShapeHandle {
    fn drop(&mut self) {
        self.world.write().unwrap().remove_shape(self.id);
    }
}

pub struct CollisionIter<'a> {
    handle: &'a ShapeHandle,
}
impl<'a> Iterator for CollisionIter<'a> {
    type Item = Collision;
    fn next(&mut self) -> Option<Collision> {
        self.handle.get_collision()
    }
}