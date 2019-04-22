mod lines;
pub mod shape;

#[cfg(test)]
mod tests {
    #[test]
    fn line_intersection() {
        use crate::lines::{Line, intersection};
        assert_eq!(intersection(&Line::new((0.5, 0.5), (1.0, 1.0)), &Line::new((0.5, 0.5), (1.0, -1.0))), Some((0.5, 0.5)));
    }
    #[test]
    fn resolution() {
        use crate::shape::Shape;
        let shapea = Shape::new(vec!((0.0, 0.0), (0.0, 1.0), (1.0, 1.0), (1.0, 0.0)));
        let shapeb = Shape::new(vec!((0.75, 0.75), (1.5, 0.75), (1.0, 1.5)));
        let output = shapea.resolve(&shapeb).unwrap();
        println!("{} {}", output.0, output.1);
        assert_eq!(shapea.moved(output).resolve(&shapeb), None);
    }
}
