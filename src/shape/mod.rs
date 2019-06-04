use std::f32::consts::PI;

use crate::{vector::Vector, lines::{Line, InEQ}};

mod shapeiters;
use shapeiters::*;

#[derive(Clone)]
pub struct Shape {
    points: Vec<Vector>,
    avg: Vector,
    displacement: Vector,
    rotation: f32,
    radius: f32,
}

impl Shape {
    pub fn new(points: Vec<Vector>, start: Vector) -> Shape {
        let mut center = Vector{x: 0.0, y: 0.0};
        for &point in points.iter() {
            center = center + point;
        }
        center = Vector{x: center.x/points.len() as f32, y: center.y/points.len() as f32};
        let mut rad = 0.0;
        for &point in points.iter() {
            let dis = (point - center).magnitude();
            if dis > rad {
                rad = dis;
            }
        }
        Shape{points: points, avg: center, displacement: start - center, rotation: 0.0, radius: rad}
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

    #[inline]
    fn get_line(&self, num: usize) -> Line {
        if num == 0 {
            return Line::new(self.get_point(self.points.len() - 1), self.get_point(0));
        }
        Line::new(self.get_point(num - 1), self.get_point(num))
    }

    #[inline]
    pub fn get_point(&self, index: usize) -> Vector {
        (self.points[index] + self.displacement).rotated_around(self.center(), self.rotation)
    }

    #[inline]
    fn get_ineq(&self, index: usize) -> InEQ {
        self.get_line(index).to_ineq(self.center())
    }

    pub fn move_by(&mut self, by: Vector) {
        self.displacement = self.displacement + by;
    }

    pub fn rotate(&mut self, angle: f32) {
        let new = self.rotation + angle;
        if new > PI {
            self.rotation = new - (PI * 2.0);
        } else if new < -PI {
            self.rotation = new + (PI * 2.0);
        } else {
            self.rotation = new;
        }
    }

    fn iter_ineq(&self) -> IneqIter {
        IneqIter::new(self)
    }

    pub fn iter_points(&self) -> PointsIter {
        PointsIter::new(self)
    }

    #[inline]
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
        let dist = (self.center() - other.center()).magnitude();
        if dist > self.radius + other.radius {
            return None;
        }
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
