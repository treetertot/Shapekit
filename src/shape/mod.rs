use crate::lines::{InEQ, Line};

pub struct Shape {
    points: Vec<(f32, f32)>,
    lines: Vec<InEQ>,
}

impl Shape {
    pub fn new(points: Vec<(f32, f32)>) -> Shape {
        let mut lines = Vec::new();
        let mut average = (0.0, 0.0);
        for (x, y) in points.iter() {
            average.0 += x;
            average.1 += y;
        }
        average = (average.0/points.len() as f32, average.1/points.len() as f32);
        lines.push(InEQ::new(Line::new(points[points.len()-1], points[0]), average));
        for i in 0..points.len()-1 {
            lines.push(InEQ::new(Line::new(points[i], points[i+1]), average));
        }
        Shape{
            points: points,
            lines: lines,
        }
    }
    pub fn resolve(&self, other: &Shape) -> Option<(f32, f32)> {
        let mut largest: Option<((f32, f32), f32)> = None;
        for point in self.points.iter() {
            match other.dist_inside(*point) {
                Some(val) => match largest {
                    Some(big) => if val.1 > big.1 {
                        largest = Some(((0.0-(val.0).0,0.0-(val.0).1), val.1));
                    },
                    None => largest = Some(((0.0-(val.0).0,0.0-(val.0).1), val.1)),
                },
                None => (),
            }
        }
        for point in other.points.iter() {
            match self.dist_inside(*point) {
                Some(val) => match largest{
                    Some(big) => if val.1 > big.1 {
                        largest = Some(val);
                    },
                    None => largest = Some(val),
                },
                None => (),
            }
        }
        Some(largest?.0)
    }
    pub fn dist_inside(&self, point: (f32, f32)) -> Option<((f32, f32), f32)> {
        let mut smallest = None;
        for eq in self.lines.iter() {
            if eq.contains(point) {
                let dist = eq.vec_and_scal_to(point);
                match smallest {
                    None => smallest = Some(dist),
                    Some(val) => if dist.1 < val.1 {
                        smallest = Some(dist)
                    },
                }
            } else {
                return None;
            }
        }
        smallest
    }
    pub fn moved(&self, (x, y): (f32, f32)) -> Shape {
        let mut new_points = self.points.clone();
        for point in &mut new_points {
            point.0 += x;
            point.1 += y;
        }
        Shape::new(new_points)
    }
    pub fn average(&self) -> (f32, f32) {
        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
        for (x, y) in self.points.iter() {
            avg_x += x;
            avg_y += y;
        }
        let len = self.points.len() as f32;
        (avg_x/len, avg_y/len)
    }
}