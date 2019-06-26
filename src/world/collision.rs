use crate::vector::Vector;
use serde::{Serialize, Deserialize};
#[derive(Clone, Debug)]
pub struct Collision<Tag: Clone> {
    pub other: Tag,
    pub resolution: Vector,
}