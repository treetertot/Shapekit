use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Line {
    slope: Option<f32>,
    intercept: f32,
}

impl Line {
    pub fn new(pointa: Vector, pointb: Vector) -> Line {
        if pointa.x == pointb.x {
            return Line{slope: None, intercept: pointa.x};
        }
        let m = (pointb.y - pointa.y)/(pointb.x - pointa.x);
        Line{slope: Some(m), intercept: pointa.y - (m * pointa.x)}
    }
    #[inline]
    pub fn to_ineq(self, point: Vector) -> InEQ {
        let greater = self.below_point(point);
        InEQ{line: self, greater: greater}
    }
    #[inline]
    pub fn normal_through(self, point: Vector) -> Line {
        match self.slope {
            Some(m) => if m == 0.0 {
                Line{slope: None, intercept: point.x}
            } else {
                let m = 1.0/m;
                Line{slope: Some(m), intercept: point.y - (m * point.x)}
            },
            None => Line{slope: Some(0.0), intercept: point.y},
        }
    }
    #[inline]
    pub fn y(self, x: f32) -> Option<f32> {
        let m = self.slope?;
        Some((m * x) + self.intercept)
    }
    #[inline]
    pub fn below_point(self, point: Vector) -> bool {
        match self.slope {
            Some(m) => point.y > (m * point.x) + self.intercept,
            None => point.x > self.intercept,
        }
    }
    #[inline]
    pub fn above_point(self, point: Vector) -> bool {
        match self.slope {
            Some(m) => point.y < (m * point.x) + self.intercept,
            None => point.x < self.intercept,
        }
    }
}

#[inline]
pub fn intersection(a: Line, b: Line) -> Option<Vector> {
    match a.slope {
        Some(m) => match b.slope {
            Some(m2) => {
                let x = (b.intercept - a.intercept)/(m - m2);
                Some(Vector{x: x, y: a.y(x)?})
            },
            None => Some(Vector{x: b.intercept, y: a.y(b.intercept)?}),
        },
        None => match b.slope {
            Some(_m) => Some(Vector{x: a.intercept, y: b.y(a.intercept)?}),
            None => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct InEQ {
    line: Line,
    greater: bool,
}

impl InEQ {
    #[inline]
    pub fn contains(self, point: Vector) -> bool {
        if self.greater {
            return !self.line.above_point(point);
        }
        !self.line.below_point(point)
    }
    #[inline]
    pub fn vec_to(self, point: Vector) -> Vector {
        let normal = self.line.normal_through(point);
        let point_on_line = intersection(self.line, normal).unwrap();
        point - point_on_line
    }
}
