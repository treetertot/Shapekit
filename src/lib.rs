mod lines;
pub mod shape;
pub mod vector;

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
        let shapea = Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(0.0, 1.0), Vector::new(1.0, 1.0), Vector::new(1.0, 0.0)), Vector::new(0.5, 0.5));
        let shapeb = Shape::new(vec!(Vector::new(0.75, 0.75), Vector::new(1.5, 0.75), Vector::new(1.0, 1.5)), Vector::new(1.833, 1.0));
        let output = shapea.resolve(&shapeb).unwrap();
        println!("{} {}", output.x, output.y);
        assert_eq!(shapea.moved(output).resolve(&shapeb), None);
    }
}
