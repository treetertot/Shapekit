mod lines;
mod shape;
pub mod vector;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn resolution() {
        use crate::vector::{Vector, MassConvert};
        use crate::world::{PhysicsWorld, ShapeHandle};
        let world = PhysicsWorld::new();
        let shape_a = world.add_shape(vec![(0.0, 0.0), (0.0, 10.0), (10.0, 10.0), (10.0, 0.0)].to_vectors(), ());
        let shape_b = world.add_shape(vec![(0.0, 0.0), (0.0, 10.0), (10.0, 10.0), (10.0, 0.0)].to_vectors(), ());
    }
    #[test]
    fn eq_test() {
        use crate::vector::Vector;
        assert_eq!(Vector{x: 1.0, y: 0.0} > Vector{x: 0.0, y: 1.0}, false);
    }
}
