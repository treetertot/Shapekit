use crate::{vector::Vector, lines::{Line, InEQ}};

mod shapeiters;
pub use shapeiters::*;

#[derive(Clone)]
pub struct Shape {
    points: Vec<Vector>,
    avg: Vector,
    displacement: Vector,
    max: Vector,
}

impl Shape {
    pub fn new(points: Vec<Vector>, start: Vector) -> Shape {
        let mut center = Vector{x: 0.0, y: 0.0};
        for &point in points.iter() {
            center = center + point;
        }
        center = Vector{x: center.x/points.len() as f32, y: center.y/points.len() as f32};
        let mut rad = 0.0;
        let mut rvec = Vector::new(0.0, 0.0);
        for &point in points.iter() {
            let vdis = point - center;
            let dis = vdis.magnitude();
            if dis > rad {
                rad = dis;
                rvec = vdis;
            }
        }
        Shape{points: points, avg: center, displacement: start - center, max: rvec.abs()}
    }

    pub fn in_place(points: Vec<Vector>) -> Shape {
        let mut center = Vector{x: 0.0, y: 0.0};
        for &point in points.iter() {
            center = center + point;
        }
        center = Vector{x: center.x/points.len() as f32, y: center.y/points.len() as f32};
        let mut rad = 0.0;
        let mut rvec = Vector::new(0.0, 0.0);
        for &point in points.iter() {
            let vdis = point - center;
            let dis = vdis.magnitude();
            if dis > rad {
                rad = dis;
                rvec = vdis;
            }
        }
        Shape{points: points, avg: center, displacement: Vector::new(0.0, 0.0), max: rvec.abs()}
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
        self.points[index] + self.displacement
    }

    #[inline]
    fn get_ineq(&self, index: usize) -> InEQ {
        self.get_line(index).to_ineq(self.center())
    }

    #[inline]
    pub fn move_by(&mut self, by: Vector) {
        self.displacement = self.displacement + by;
    }

    fn iter_ineq(&self) -> IneqIter {
        IneqIter::new(self)
    }

    pub fn iter_points(&self) -> PointsIter {
        PointsIter::new(self)
    }

    pub fn bottom_left(&self) -> Vector {
        let mut least = self.get_point(0);
        for pt in self.iter_points().skip(1) {
            if pt.x < least.x {
                least.x = pt.x;
            }
            if pt.y < least.y {
                least.y = pt.y;
            }
        }
        least
    }

    pub fn max_test(&self, other: &Shape) -> bool { // true if could collide
        let (x, y) = (other.center() - self.center()).abs().to_tuple();
        let combined = self.max + other.max;
        x < combined.x && y < combined.y
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
    
    #[inline]
    pub fn resolve(&self, other: &Shape) -> Option<Vector> {
        if !self.max_test(other) {
            return None;
        }
        for point in other.iter_points() {
            if let Some(dist) = self.dist_inside(point) {
                return Some(dist);
            }
        }
        for point in self.iter_points() {
            if let Some(dist) = other.dist_inside(point) {
                return Some(dist * -1.0);
            }
        }
        None
    }
}