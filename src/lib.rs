mod lines;
pub mod shape;
pub mod processing;

#[cfg(test)]
mod tests {
    #[test]
    fn resolution() {
        use amethyst::core::math::Point2;
        use crate::shape::Shape;
        let a = Shape::new(vec![Point2::new(0.0, 0.0), Point2::new(0.0, 10.0), Point2::new(10.0, 10.0), Point2::new(10.0, 0.0)]);
        let b = Shape::new(vec![Point2::new(7.4, 7.5), Point2::new(7.4, 17.5), Point2::new(17.4, 17.5), Point2::new(17.4, 7.5)]);
        assert!(b.collide(&a).is_some());
    }
    #[test]
    fn eq_test() {
        use amethyst::core::math::Vector2;
        assert_eq!(Vector2::new(1.0, 0.0) > Vector2::new(0.0, 1.1), false);
    }
    #[test]
    fn raycast_test() {
        use amethyst::core::math::Point2;
        use crate::shape::Shape;
        use crate::processing::Raycast;
        let shape_a = Shape::new(
            vec![Point2::new(0.0, 0.0), Point2::new(0.0, 10.0), Point2::new(10.0, 10.0), Point2::new(10.0, 0.0)]
        );
        assert_ne!(Raycast::new(&[(shape_a, ())], Point2::new(-1.0, -1.0), 1.0).next(), None);
    }
    #[test]
    fn transformation() {
        use amethyst::core::math::{Point2, Translation3, UnitQuaternion, Vector3};
        use amethyst::core::transform::Transform;
        use crate::shape::Shape;
        let mut a = Shape::new(vec![Point2::new(0.0, 0.0), Point2::new(0.0, 10.0), Point2::new(10.0, 10.0), Point2::new(10.0, 0.0)]);
        a.set_transformation(&Transform::default());
        let mut b = a.clone();
        let transform = Transform::new(Translation3::new(7.4, 7.5, 0.), UnitQuaternion::identity(), Vector3::new(1., 1., 1.));
        b.set_transformation(&transform);
        assert!(b.collide(&a).is_some());
    }
}
