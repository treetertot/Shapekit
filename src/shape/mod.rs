use crate::{vector::Vector, lines::{Line, InEQ}};

mod shapeiters;
use shapeiters::*;

#[derive(Clone)]
pub struct Shape {
    points: Vec<Vector>,
    avg: Vector,
    displacement: Vector,
}

impl Shape {
    pub fn new(points: Vec<Vector>, start: Vector) -> Shape {
        let mut center = Vector{x: 0.0, y: 0.0};
        for point in points.iter() {
            center = center + *point;
        }
        center = Vector{x: center.x/points.len() as f32, y: center.y/points.len() as f32};
        Shape{points: points, avg: center, displacement: start - center}
    }

    pub fn from_tuples(tuples: Vec<(f32, f32)>) -> Shape {
        let mut new = Vec::new();
        for i in 0..(tuples.len() - 1) {
            new.push(Vector::from_tuple(tuples[i]));
        }
        Shape::new(new, Vector::from_tuple(tuples[tuples.len()-1]))
    }

    pub fn center(&self) -> Vector {
        self.avg + self.displacement
    }

    fn get_line(&self, num: usize) -> Line {
        if num == 0 {
            return Line::new(self.points[self.points.len() - 1] + self.displacement, self.points[0] + self.displacement);
        }
        Line::new(self.points[num - 1] + self.displacement, self.points[num] + self.displacement)
    }

    pub fn get_point(&self, index: usize) -> Vector {
        self.points[index] + self.displacement
    }

    fn get_ineq(&self, index: usize) -> InEQ {
        self.get_line(index).to_ineq(self.center())
    }

    pub fn move_by(&mut self, by: Vector) {
        self.displacement = self.displacement + by;
    }

    fn iter_ineq(&self) -> IneqIter {
        IneqIter::new(self)
    }

    pub fn iter_points(&self) -> PointsIter {
        PointsIter::new(self)
    }

    fn dist_inside(&self, point: Vector) -> Option<Vector> {
        let mut smallest = None;
        for ieq in self.iter_ineq() {
            if ieq.contains(point) {
                let dist = ieq.vec_to(point);
                match smallest {
                    None => smallest = Some(dist),
                    Some(val) => if dist.magnitude() < val.magnitude() {
                        smallest = Some(dist)
                    },
                }
            } else {
                return None;
            }
        }
        smallest
    }

    pub fn resolve(&self, other: &Shape) -> Option<Vector> {
        for point in other.iter_points() {
            if let Some(dist) = self.dist_inside(point) {
                return Some(dist);
            }
        }
        for point in self.iter_points() {
            if let Some(dist) = other.dist_inside(point) {
                return Some(dist * -1.0)
            }
        }
        None
    }
}
