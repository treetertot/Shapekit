use amethyst::core::ecs::prelude::*;
use amethyst::core::transform::Transform;
use amethyst::core::math::Vector2;

use smallvec::SmallVec;

use std::marker::PhantomData;

use crate::shape::Shape;
use crate::lines::CollisionVector;

pub struct ShapeSync;
impl<'a> System<'a> for ShapeSync {
    type SystemData = (ReadStorage<'a, Transform>, WriteStorage<'a, Shape>);

    fn run(&mut self, (transforms, mut shapes): Self::SystemData) {
        for (transform, shape) in (&transforms, &mut shapes).join() {
            shape.set_transformation(transform);
        }
    }
}

pub struct Collisions<T> {
    aggregate: Vector2<f32>,
    collisions: SmallVec<[(CollisionVector, T); 1]>
}
impl<T> Collisions<T> {
    pub fn new() -> Self {
        Collisions {
            aggregate: Vector2::new(0., 0.),
            collisions: SmallVec::new()
        }
    }
    pub fn resolution(&self) -> Vector2<f32> {
        self.aggregate
    }
    pub fn collisions(&self) -> &[(CollisionVector, T)] {
        &self.collisions
    }
}
impl<T> Component for Collisions<T> where
T: 'static + Send + Sync {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct ColliderSys<T>{
    dud: PhantomData<T>
}
impl<'a, T> System<'a> for ColliderSys<T> where
T: 'static + Send + Sync + Component + Clone {
    type SystemData = (ReadStorage<'a, Shape>, WriteStorage<'a, Collisions<T>>, Entities<'a>, ReadStorage<'a, T>);

    fn run(&mut self, (shapes, mut collisions, entities, tags): Self::SystemData) {
        for (shape_a, collision_out, id_a) in (&shapes, &mut collisions, &entities).join() {
            let mut record = SmallVec::new();
            let mut aggregate = Vector2::new(0., 0.);
            for (shape_b, _id_b, tag) in (&shapes, &entities, &tags).join().filter(|(_, id, _)| *id != id_a) {
                if let Some(vector) = shape_a.collide(shape_b) {
                    if let CollisionVector::Resolve(resolution) = vector {
                        aggregate += resolution;
                    }
                    record.push((vector, tag.clone()));
                }
            }
            *collision_out = Collisions {
                aggregate,
                collisions: record,
            }
        }
    }
}
impl<T> ColliderSys<T> {
    pub fn new() -> Self {
        ColliderSys {
            dud: PhantomData
        }
    }
}