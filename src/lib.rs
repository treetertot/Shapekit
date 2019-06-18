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
        let mut shapa = Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(50.0, 70.0)), Vector::new(0.0, 0.0));
        let shapb = Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(50.0, 70.0)), Vector::new(51.0, 51.0));
        let res = shapa.resolve(&shapb).unwrap();
        println!("{}", res);
        shapa.move_by(res);
        let res = shapa.resolve(&shapb).unwrap();
        println!("{}", res);
    }
    #[test]
    fn checker() {
        use crate::shape::Shape;
        use crate::vector::Vector;
        use crate::world::compare;
        let mut out = Vec::new();
        compare(&mut out, &Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(50.0, 70.0)), Vector::new(0.0, 0.0)), &Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(50.0, 70.0)), Vector::new(51.0, 51.0)), 0, 1);
        assert_eq!(2, out.len());
    }
}
