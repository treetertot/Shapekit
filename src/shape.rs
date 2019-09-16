use crate::vector::Vector;
mod shapeiters;
use shapeiters::*;
use std::f32;
pub struct Shape {
    pub points: Vec<Vector>,
    pub center: Vector,
    pub displacement: Vector,
}
impl Shape {
    pub fn new(points: Vec<Vector>) -> Shape {
        let mut avg = Vector { x: 0.0, y: 0.0 };
        for &point in &points {
            avg += point;
        }
        if points.len() != 0 {
            avg = avg / (points.len() as f32);
        }
        Shape {
            points: points,
            center: avg,
            displacement: Vector { x: 0.0, y: 0.0 },
        }
    }
    pub fn iter_points<'a>(&'a self) -> PointsIter<'a> {
        PointsIter {
            points: self.points.iter(),
            displacement: self.displacement,
        }
    }
    fn iter_sides<'a>(&'a self) -> SidesIter<'a> {
        let mut iter = self.iter_points().peekable();
        match iter.peek() {
            Some(&first) => SidesIter {
                points: iter,
                center: self.center + self.displacement,
                first: first,
            },
            None => SidesIter {
                points: iter,
                center: Vector::default(),
                first: Vector::default(),
            },
        }
    }
    fn dist_inside(&self, point: Vector) -> Option<Vector> {
        let mut out: Option<(Vector, f32)> = None;
        for side in self.iter_sides() {
            let dist = side.distance(point)?;
            let mag = dist.magnitude();
            match out {
                Some(val) => {
                    if mag < val.1 {
                        out = Some((dist, mag))
                    }
                }
                None => out = Some((dist, mag)),
            }
        }
        Some(out?.0)
    }
    pub fn resolve(&self, other: &Shape) -> Option<Vector> {
        let mut collisions = Vec::new();
        for point in self.iter_points() {
            if let Some(dist) = other.dist_inside(point) {
                collisions.push(dist);
            }
        }
        for point in other.iter_points() {
            if let Some(dist) = self.dist_inside(point) {
                collisions.push(dist * -1.0);
            }
        }
        let mut max = *collisions.first()?;
        for res in collisions.into_iter().skip(1) {
            if res.magnitude() > max.magnitude() {
                max = res;
            }
        }
        Some(max)
    }
}
