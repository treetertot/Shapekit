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

    pub fn iter_points(&self) -> IneqIter {
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
        let mut largest: Option<Vector> = None;
        for &point in self.points.iter() {
            match other.dist_inside(point) {
                Some(val) => match largest {
                    Some(big) => if val.magnitude() > big.magnitude() {
                        largest = Some(Vector{x: 0.0, y: 0.0} - val)
                    }
                    None => largest = Some(Vector{x: 0.0, y: 0.0} - val)
                }
                None => ()
            }
        }
        for &point in other.points.iter() {
            match self.dist_inside(point) {
                Some(val) => match largest {
                    Some(big) => if val.magnitude() > big.magnitude() {
                        largest = Some(val)
                    }
                    None => largest = Some(val)
                }
                None => ()
            }
        }
        largest
    }
}
