mod shapeiters;
use crate::lines::*;
use shapeiters::*;
use std::f32;
use std::slice::Iter;
use std::borrow::Borrow;
use std::cmp::PartialEq;
use amethyst::{
    core::{
        math::{Point2, Vector2, Point4},
        transform::Transform
    },
    assets::{PrefabData},
    ecs::{
        storage::DenseVecStorage,
        Component,
        Entity,
        WriteStorage
    },
    Error
};
use serde::{Serialize, Deserialize};

pub use crate::lines::CollisionVector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapePrefab {
    pub points: Vec<Point2<f32>>
}
impl<'a> PrefabData<'a> for ShapePrefab {
    type SystemData = WriteStorage<'a, Shape>;

    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        shapes: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<(), Error> {
        shapes.insert(entity, Shape::new(self.points.clone())).map(|_| ())?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Shape {
    points: Vec<Point2<f32>>,
    moved_points: Vec<Point2<f32>>,
    center: Point2<f32>,
    moved_center: Point2<f32>,
}
impl Shape {
    pub fn new<PointType: Borrow<Point2<f32>>, I: IntoIterator<Item=PointType>>(points: I) -> Shape {
        let mut avg = Vector2::new(0., 0.);
        let mut iter = points.into_iter().peekable();
        let size = if iter.size_hint().0 != 0 {
            iter.size_hint().0 * 2
        } else {
            8
        };
        let first = match iter.peek() {
            Some(value) => Vector2::new(value.borrow().x, value.borrow().y),
            None => return Shape {
                points: Vec::new(),
                moved_points: Vec::new(),
                moved_center: Point2::new(0. ,0.),
                center: Point2::new(0., 0.)
            }
        };
        let mut points = Vec::with_capacity(size);
        while let Some(point) = iter.next() {
            let point = point.borrow();
            let next = match iter.peek() {
                Some(nxt) => Vector2::new(nxt.borrow().x, nxt.borrow().y),
                None => first
            };
            let mid_mid = point + next;
            let mid = Point2::new(mid_mid.x/2., mid_mid.y/2.);
            println!("point: {:?}, next: {:?}, mid: {:?}", point, next, mid);
            avg += Vector2::new(point.x, point.y);
            points.push(*point);
            points.push(mid);
            
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
    
    fn dist_inside(&self, point: Point2<f32>) -> Option<CollisionVector> {
        let mut out: Option<(CollisionVector, f32)> = None;
        for side in self.iter_sides() {
            let dist = side.distance(point)?;
            match dist {
                CollisionVector::Touch(_) => return Some(dist),
                CollisionVector::Resolve(resolution) => {
                    let mag = resolution.magnitude();
                    match &out {
                        Some(val) => {
                            if mag < val.1 {
                                out = Some((dist, mag));
                            }
                        }
                        None => out = Some((dist, mag))
                    }
                }
            }
        }
        Some(out?.0)
    }
    pub fn collide(&self, other: &Shape) -> Option<CollisionVector> {
        let mut result = None;
        for res in self.iter_points()
        .filter_map(|point| other.dist_inside(*point))
        .chain(
            other
                .iter_points()
                .filter_map(|point| Some(other.dist_inside(*point)?.flip())),
        ) {
            match &result {
                None => result = Some((res.magnitude(), res)),
                Some((mag, _old_res)) => {
                    match res {
                        CollisionVector::Touch(_) => return Some(res),
                        CollisionVector::Resolve(res_vec) => {
                            let new_mag = res_vec.magnitude();
                            if *mag < new_mag {
                                result = Some((new_mag, res))
                            }
                        }
                    }
                }
            }
        }
        Some(result?.1)
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
    pub fn transform<F: FnMut(&Point2<f32>) -> Point2<f32>>(&mut self, mut func: F) {
        for (point, p_out) in self.points.iter().map(&mut func).zip(self.moved_points.iter_mut()) {
            *p_out = point;
        }
        self.moved_center = func(&self.center);
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