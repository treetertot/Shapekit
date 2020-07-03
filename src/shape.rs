mod shapeiters;
use crate::lines::*;
use shapeiters::*;
use std::f32;
use std::slice::Iter;
use std::cmp::PartialEq;
use amethyst_core::math::{Point2, Vector2, Point4};
use amethyst_core::transform::Transform;
use amethyst_core::ecs::prelude::*;

#[derive(Debug, Clone)]
pub struct Shape {
    points: Vec<Point2<f32>>,
    moved_points: Vec<Point2<f32>>,
    center: Point2<f32>,
    moved_center: Point2<f32>,
}
impl Shape {
    pub fn new(points: Vec<Point2<f32>>) -> Shape {
        let mut avg = Vector2::new(0., 0.);
        for &point in &points {
            avg += Vector2::new(point.x, point.y);
        }
        if points.len() != 0 {
            avg = avg / (points.len() as f32);
        }
        let center = Point2::from(avg);
        Shape {
            points: points.clone(),
            moved_points: points,
            moved_center: center,
            center
        }
    }
    pub fn iter_points<'a>(&'a self) -> Iter<'a, Point2<f32>> {
        self.moved_points.iter()
    }
    fn iter_sides<'a>(&'a self) -> SidesIter<'a> {
        let mut iter = self.iter_points().peekable();
        match iter.peek() {
            Some(&first) => SidesIter {
                points: iter,
                center: self.moved_center, //apply transform
                first: *first,
            },
            None => SidesIter {
                points: iter,
                center: Point2::new(0., 0.),
                first: Point2::new(0., 0.),
            },
        }
    }
    fn dist_inside(&self, point: Point2<f32>) -> Option<Vector2<f32>> {
        let mut out: Option<(Vector2<f32>, f32)> = None;
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
    pub fn resolve(&self, other: &Shape) -> Option<Vector2<f32>> {
        Some(
            self.iter_points()
                .filter_map(|point| other.dist_inside(*point))
                .chain(
                    other
                        .iter_points()
                        .filter_map(|point| Some(other.dist_inside(*point)? * -1.0)),
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
    pub fn receive_ray(&self, ray: Line, normal: InEq) -> Option<Vector2<f32>> {
        self.iter_sides()
            .mangle()
            .filter_map(|(line, start, end)| line.intersection_segment(&ray, start, end))
            .filter(|&pt| normal.contains(pt))
            .fold(None, |prev, new_val| match prev {
                Some(prev) => {
                    if Vector2::new(new_val.x, new_val.y).magnitude() < Vector2::new(prev.x, prev.y).magnitude() {
                        Some(Vector2::new(new_val.x, new_val.y))
                    } else {
                        Some(prev)
                    }
                }
                None => Some(Vector2::new(new_val.x, new_val.y)),
            })
    }
    pub fn set_transformation(&mut self, transform: &Transform) {
        let mat = transform.matrix();
        self.transform(|pt| {
            let tpoint = Point4::new(pt.x, pt.y, 1., 1.);
            let tformed = mat * tpoint;
            Point2::new(tformed.x, tformed.y)
        });
    }
    pub fn transform<F: FnMut(&Point2<f32>) -> Point2<f32>>(&mut self, func: F) {
        for (point, p_out) in self.points.iter().map(func).zip(self.moved_points.iter_mut()) {
            *p_out = point;
        }
    }
}

impl PartialEq for Shape {
    fn eq(&self, right: &Self) -> bool {
        self.moved_points == right.moved_points
    }
}

impl Component for Shape {
    type Storage = DenseVecStorage<Self>;
}