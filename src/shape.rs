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
        Some(
            self.iter_points()
                .filter_map(|point| other.dist_inside(point))
                .chain(
                    other
                        .iter_points()
                        .filter_map(|point| Some(other.dist_inside(point)? * -1.0)),
                )
                .fold(None, |prev, new_pt| match prev {
                    Some((mag, vec)) => {
                        let new_mag = new_pt.magnitude();
                        if mag < new_pt.magnitude() {
                            Some((new_mag, new_pt))
                        } else {
                            Some((mag, vec))
                        }
                    }
                    None => Some((new_pt.magnitude(), new_pt)),
                })?
                .1,
        )
    }
}
