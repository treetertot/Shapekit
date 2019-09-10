use crate::shape::Shape;
use crate::vector::Vector;
use std::mem;
use std::ops::Drop;
use std::sync::{Arc, Mutex, RwLock};
struct Item<T: Send + Sync + Clone> {
    shape: Shape,
    tag: T,
    collisions: Option<Vec<(Vector, T)>>,
}
pub struct PhysicsWorld<T: Send + Sync + Clone>(Arc<RwLock<Vec<(usize, Mutex<Item<T>>)>>>);
impl<T: Send + Sync + Clone> PhysicsWorld<T> {
    pub fn new() -> PhysicsWorld<T> {
        PhysicsWorld(Arc::new(RwLock::new(Vec::new())))
    }
    pub fn add_shape(&self, points: Vec<Vector>, tag: T) -> ShapeHandle<T> {
        let shape = Shape::new(points);
        let mut guard = self.0.write().unwrap();
        let count = match guard.last() {
            Some((num, _)) => num + 1,
            None => 0,
        };
        let container = (
            count,
            Mutex::new(Item {
                shape: shape,
                tag: tag,
                collisions: None,
            }),
        );
        guard.push(container);
        ShapeHandle {
            parent: self.0.clone(),
            id: count,
        }
    }
}
pub struct ShapeHandle<T: Send + Sync + Clone> {
    parent: Arc<RwLock<Vec<(usize, Mutex<Item<T>>)>>>,
    id: usize,
}
impl<T: Send + Sync + Clone> ShapeHandle<T> {
    pub fn collisions(&self) -> Vec<(Vector, T)> {
        let readable = self.parent.read().unwrap();
        let index = get_index(&readable, self.id);
        let mut guard = readable[index].1.lock().unwrap();
        match &mut guard.collisions {
            Some(_cols) => {
                let mut capture = None;
                mem::swap(&mut guard.collisions, &mut capture);
                return capture.unwrap();
            }
            None => (),
        }
        mem::drop(guard);
        mem::drop(readable);
        let mut writeable = self.parent.write().unwrap();
        for (indexa, mut itema) in writeable
            .iter()
            .map(|(_id, mutitem)| mutitem.lock().unwrap())
            .enumerate()
        {
            match itema.collisions {
                Some(_) => (),
                None => itema.collisions = Some(Vec::new()),
            }
            for mut itemb in writeable
                .iter()
                .map(|(_id, mutitem)| mutitem.lock().unwrap())
                .skip(indexa + 1)
            {
                if let Some(res) = itema.shape.resolve(&itemb.shape) {
                    match &mut itema.collisions {
                        Some(list) => list.push((res, itemb.tag.clone())),
                        None => itema.collisions = Some(vec![(res, itemb.tag.clone())]),
                    }
                    match &mut itemb.collisions {
                        Some(list) => list.push((res * -1.0, itema.tag.clone())),
                        None => itema.collisions = Some(vec![(res * -1.0, itema.tag.clone())]),
                    }
                }
            }
        }
        let mut capture = None;
        mem::swap(
            &mut writeable[index].1.lock().unwrap().collisions,
            &mut capture,
        );
        match capture {
            Some(val) => val,
            None => Vec::new(),
        }
    }
    pub fn move_by(&self, v: Vector) {
        let readable = self.parent.read().unwrap();
        let mut guard = readable[get_index(&readable, self.id)].1.lock().unwrap();
        guard.shape.displacement += v;
    }
    pub fn points(&self) -> Vec<Vector> {
        let readable = self.parent.read().unwrap();
        let guard = readable[get_index(&readable, self.id)].1.lock().unwrap();
        guard
            .shape
            .points
            .clone()
            .into_iter()
            .map(|point| point + guard.shape.displacement)
            .collect()
    }
    pub fn tag(&self) -> T {
        let readable = self.parent.read().unwrap();
        let guard = readable[get_index(&readable, self.id)].1.lock().unwrap();
        guard.tag.clone()
    }
}
impl<T: Send + Sync + Clone> Drop for ShapeHandle<T> {
    fn drop(&mut self) {
        let mut writeable = self.parent.write().unwrap();
        let index = get_index(&writeable, self.id);
        writeable.remove(index);
    }
}
fn get_index<T: Send + Sync + Clone>(list: &Vec<(usize, Mutex<Item<T>>)>, id: usize) -> usize {
    match list.binary_search_by(|(num, _)| num.cmp(&id)) {
        Ok(val) => val,
        Err(_) => panic!("Shape not found"),
    }
}
