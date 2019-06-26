use crate::vector::Vector;

#[derive(Clone, Debug)]
pub struct Collision<Tag: Clone> {
    pub other: Tag,
    pub resolution: Vector,
}