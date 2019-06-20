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
        use rand::Rng;
        let mut gen = rand::thread_rng();
        for _ in 0..100 {
            let mut out = Vec::new();
            let x = gen.gen_range(-99.999, 100.0);
            let y = gen.gen_range(-99.999, 100.0);
            compare(&mut out, &Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(100.0, 100.0), Vector::new(0.0, 100.0)), Vector::new(0.0, 0.0)), &Shape::new(vec!(Vector::new(0.0, 0.0), Vector::new(100.0, 0.0), Vector::new(100.0, 100.0), Vector::new(0.0, 100.0)), Vector::new(x, y)), 0, 1);
            assert_eq!(2, out.len());
        }
    }
    #[test]
    fn in_handle() {
        use crate::world::WorldHandle;
        use rand::Rng;
        let mut gen = rand::thread_rng();
        let w = WorldHandle::new();
        for _ in 0..100 {
            let x = gen.gen_range(-99.999, 100.0);
            let y = gen.gen_range(-99.999, 100.0);
            let s1 = w.add_shape(vec!((0.0, 0.0), (100.0, 0.0), (100.0, 100.0), (0.0, 100.0)), (0.0, 0.0));
            let s2 = w.add_shape(vec!((0.0, 0.0), (100.0, 0.0), (100.0, 100.0), (0.0, 100.0)), (x, y));
            let mut it = s1.collisions();
            assert_ne!(None, it.next());
            let mut it = s2.collisions();
            assert_ne!(None, it.next());
            println!("off to a start");
        }
    }
}
use std::cmp::PartialEq;
impl PartialEq for world::collision::Collision {
    fn eq(&self, other: &Self) -> bool {
        self.other == other.other && self.resolution == other.resolution
    }
}