mod lines;
mod shape;
pub mod vector;
pub mod world;

#[cfg(test)]
mod tests {
    #[test]
    fn line_intersection() {
        use crate::lines::{Line, intersection};
        use crate::vector::Vector;
        assert_eq!(intersection(Line::new(Vector::new(0.5, 0.5), Vector::new(1.0, 1.0)), Line::new(Vector::new(0.5, 0.5), Vector::new(1.0, -1.0))), Some(Vector::new(0.5, 0.5)));
    }
    #[test]
    fn resolution() {
        use crate::shape::Shape;
        use crate::vector::Vector;
        let shapa = Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(50.0, 70.0)), Vector::new(0.0, 0.0));
        let _shapb = Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(50.0, 70.0)), Vector::new(0.0, 0.0));
        println!("c{}", shapa.center());
    }
}
