use crate::vector::Vector;
use serde::{Serialize, Deserialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Collision {
    pub other: usize,
    pub resolution: Vector,
}