mod lines;
pub mod shape;
pub mod vector;
pub mod world;
pub mod iterator_based;

#[cfg(test)]
mod tests {
    #[test]
    fn resolution() {
        use crate::vector::MassConvert;
        use crate::world::PhysicsWorld;
        let world = PhysicsWorld::new();
        let shape_a = world.add_shape(
            vec![(0.0, 0.0), (0.0, 10.0), (10.0, 10.0), (10.0, 0.0)].to_vectors(),
            (),
        );
        let _shape_b = world.add_shape(
            vec![(7.4, 7.5), (7.4, 17.5), (17.4, 17.5), (17.4, 7.5)].to_vectors(),
            (),
        );
        assert_ne!(shape_a.collisions().into_iter().next(), None);
    }
    #[test]
    fn eq_test() {
        use crate::vector::Vector;
        assert_eq!(Vector { x: 1.0, y: 0.0 } > Vector { x: 0.0, y: 1.1 }, false);
    }
    #[test]
    fn raycast_test() {
        use crate::vector::MassConvert;
        use crate::vector::Vector;
        use crate::world::PhysicsWorld;
        let world = PhysicsWorld::new();
        let _shape_a = world.add_shape(
            vec![(0.0, 0.0), (0.0, 10.0), (10.0, 10.0), (10.0, 0.0)].to_vectors(),
            (),
        );
        assert_ne!(world.raycast_nearest(Vector::new(-1.0, -1.0), 1.0), None);
    }
}
