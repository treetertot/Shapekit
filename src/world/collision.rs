use crate::vector::Vector;
#[derive(Clone)]
pub struct Collision {
    pub other: usize,
    pub resolution: Vector,
}