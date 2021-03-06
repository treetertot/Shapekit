use amethyst::core::math::{Point2, Vector2};
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct Line {
    slope: Option<f32>,
    constant: f32,
}
impl Line {
    pub fn through(a: Point2<f32>, b: Point2<f32>) -> Line {
        if a.x == b.x {
            Line {
                slope: None,
                constant: a.x,
            }
        } else {
            let m = (a.y - b.y) / (a.x - b.x);
            Line {
                slope: Some(m),
                constant: a.y - (m * a.x),
            }
        }
    }
    pub fn y(&self, x: f32) -> Option<f32> {
        Some((self.slope? * x) + self.constant)
    }
    pub fn initialize(self, point: Point2<f32>) -> InEq {
        InEq {
            greater: match self.y(point.x) {
                Some(val) => point.y > val,
                None => point.x > self.constant,
            },
            line: self,
        }
    }
    pub fn normal_through(&self, point: Point2<f32>) -> Line {
        match self.slope {
            Some(val) => {
                if val == 0.0 {
                    Line {
                        slope: None,
                        constant: point.x,
                    }
                } else {
                    let m = 1.0 / val;
                    Line {
                        slope: Some(m),
                        constant: point.y - (m * point.x),
                    }
                }
            }
            None => Line {
                slope: Some(0.0),
                constant: point.y,
            },
        }
    }
    pub fn intersection(self, b: &Line) -> Option<Point2<f32>> {
        match self.slope {
            Some(m) => match b.slope {
                Some(m2) => {
                    let x = (b.constant - self.constant) / (m - m2);
                    Some(Point2::new(x, self.y(x)?))
                }
                None => Some(
                    Point2::new(b.constant, self.y(b.constant)?)
                ),
            },
            None => match b.slope {
                Some(_m) => Some(
                    Point2::new(self.constant, b.y(self.constant)?)
                ),
                None => None,
            },
        }
    }
    pub fn intersection_segment(self, other: &Line, start: Point2<f32>, end: Point2<f32>) -> Option<Point2<f32>> {
        let isect = self.intersection(other)?;
        if ((start.x <= isect.x && isect.x <= end.x) || (start.x >= isect.x && isect.x >= end.x))
            && ((start.y <= isect.y && isect.y <= end.y)
                || (start.y >= isect.y && isect.y >= end.y))
        {
            return Some(isect);
        }
        None
    }
}

#[derive(Clone, Copy)]
pub struct InEq {
    line: Line,
    greater: bool,
}
impl InEq {
    pub fn contains(&self, point: Point2<f32>) -> bool {
        match self.line.y(point.x) {
            Some(val) => {
                if self.greater {
                    point.y > val
                } else {
                    point.y < val
                }
            }
            None => {
                if self.greater {
                    point.x > self.line.constant
                } else {
                    point.x < self.line.constant
                }
            }
        }
    }
    fn touches(&self, point: Point2<f32>) -> TouchResult {
        if self.contains(point) {
            TouchResult::Contain
        } else if match self.line.y(point.x) {
            Some(val) => {
                if self.greater {
                    point.y == val
                } else {
                    point.y == val
                }
            }
            None => {
                if self.greater {
                    point.x == self.line.constant
                } else {
                    point.x == self.line.constant
                }
            }
        } {
            TouchResult::Touch
        } else {
            TouchResult::None
        }
    }
    pub fn distance(&self, point: Point2<f32>) -> Option<CollisionVector> {
        match self.touches(point) {
            TouchResult::None => None,
            TouchResult::Touch => Some(CollisionVector::Touch(self.normal_vector())),
            TouchResult::Contain => Some(
                CollisionVector::Resolve(
                    self.line
                        .normal_through(point)
                        .intersection(&self.line)
                        .unwrap()
                        - point,
                )
            )
        }
    }
    fn normal_vector(&self) -> Vector2<f32> {
        let naive = match self.line.slope {
            Some(m) => {
                let rand_length = Vector2::new(m, 1.);
                rand_length / rand_length.magnitude()
            },
            None => Vector2::new(1., 0.)
        };
        match self.greater {
            true => naive,
            false => -naive,
        }
    }
}

enum TouchResult {
    Touch,
    Contain,
    None
}

#[derive(Debug, Clone)]
pub enum CollisionVector {
    Touch(Vector2<f32>),
    Resolve(Vector2<f32>)
}
impl CollisionVector {
    pub fn flip(&self) -> CollisionVector {
        match self {
            Self::Touch(val) => Self::Touch(val * -1.),
            Self::Resolve(val) => Self::Touch(val * -1.)
        }
    }
}
impl Deref for CollisionVector {
    type Target = Vector2<f32>;

    fn deref(&self) -> &Vector2<f32> {
        match self {
            Self::Touch(val) => val,
            Self::Resolve(val) => val
        }
    }
}