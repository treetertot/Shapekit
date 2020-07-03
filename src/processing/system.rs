use amethyst_core::ecs::prelude::*;
use amethyst_core::transform::Transform;
use crate::shape::Shape;

pub struct ShapeSync;
impl<'a> System<'a> for ShapeSync {
    type SystemData = (ReadStorage<'a, Transform>, WriteStorage<'a, Shape>);

    fn run(&mut self, (transforms, mut shapes): Self::SystemData) {
        for (transform, shape) in (&transforms, &mut shapes).join() {
            shape.set_transformation(transform);
        }
    }
}