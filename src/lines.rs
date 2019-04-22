pub struct Line {
    slope: Option<f32>,
    intercept: f32,
}

impl Line {
    pub fn new((ax, ay): (f32, f32), (bx, by): (f32, f32)) -> Line {
        if ax == bx {
            return Line{slope: None, intercept: ax};
        }
        let m = (by - ay)/(bx - ax);
        Line{slope: Some(m), intercept: ay - (m * ax)}
    }
    #[inline]
    pub fn normal_through(&self, (px, py): (f32, f32)) -> Line {
        match self.slope {
            Some(m) => if m == 0.0 {
                Line{slope: None, intercept: px}
            } else {
                let m = 1.0/m;
                Line{slope: Some(m), intercept: py - (m * px)}
            },
            None => Line{slope: Some(0.0), intercept: py},
        }
    }
    #[inline]
    pub fn y(&self, x: f32) -> Option<f32> {
        let m = self.slope?;
        Some((m * x) + self.intercept)
    }
    #[inline]
    fn point_is_above(&self, (x, y): (f32, f32)) -> bool {
        match self.slope {
            Some(m) => y > (m * x) + self.intercept,
            None => x > self.intercept,
        }
    }
}

#[inline]
pub fn intersection(a: &Line, b: &Line) -> Option<(f32, f32)> {
    match a.slope {
        Some(m) => match b.slope {
            Some(m2) => {
                let x = (b.intercept - a.intercept)/(m - m2);
                Some((x, a.y(x)?))
            },
            None => Some((b.intercept, a.y(b.intercept)?)),
        },
        None => match b.slope {
            Some(_m) => Some((a.intercept, b.y(a.intercept)?)),
            None => None,
        }
    }
}

pub struct InEQ {
    line: Line,
    greater: bool,
}

impl InEQ {
    pub fn new(line: Line, init: (f32, f32)) -> InEQ {
        let greater = line.point_is_above(init);
        InEQ{line: line, greater: greater}
    }
    #[inline]
    pub fn contains(&self, point: (f32, f32)) -> bool {
        if self.greater {
            return self.line.point_is_above(point);
        }
        !self.line.point_is_above(point)
    }
    #[inline]
    pub fn vec_and_scal_to(&self, point: (f32, f32)) -> ((f32, f32), f32) {
        let normal = self.line.normal_through(point);
        let isect = intersection(&self.line, &normal).unwrap();
        let vector = (isect.0, isect.1);
        let scalar = (vector.0.powf(2.0) + vector.1.powf(2.0)).powf(0.5);
        (vector, scalar)
    }
}