use core::ops::Add;
use core::ops::Sub;
use core::ops::Neg;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    curve: EllipticCurve,
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        if self == other {
            let m = (3 * self.x * self.x + self.curve.a) / (2 * self.y);
            let r_x = m * m - self.x - other.x;
            let r_y = -(self.y - m * (r_x - self.x));
            
            Point {
                curve: self.curve,
                x: r_x,
                y: r_y,
            }
            
        } else {
            let m = (self.y - other.y) / (self.x - other.x);
            let r_x = m * m - self.x - other.x;
            let r_y = -(self.y + m * (r_x - self.x));
            
            Point {
                curve: self.curve,
                x: r_x,
                y: r_y,
            }
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            curve: self.curve,
            x: self.x,
            y: -self.y,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EllipticCurve {
    a: i32,
    b: i32,
}

impl EllipticCurve {
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    pub fn random() -> Self {
        Self {
            a: 0, // TODO: initialize with random value
            b: 0, // TODO: initialize with random value
        }
    }
    pub fn has_point(self, x: i32, y:i32) -> bool {
        y*y == x*x*x + self.a * x + self.b
    }
    pub fn point(self, x: i32, y: i32) -> Point {
        // TODO: check if the point is in the curve
        // TODO: panic! is the check fails
        // panic!("the point ({}, {}) does not belong to this curve", x, y);
        if self.has_point(x, y) {
            Point { curve: self, x, y }    
        } else {
            panic!("point ({}, {}), not in curve y^2 = x^3 + {}x + {}", x, y, self.a, self.b)
        }
        
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add() {
        let curve = EllipticCurve::new(-7, 10);

        let p = curve.point(-1, 4);
        let q = curve.point(1, 2);

        assert_eq!(p + q, Point { curve, x: 1, y: -2 },);
    }

    #[test]
    #[should_panic]
    fn invalid_point() {
        let curve = EllipticCurve::new(-7, 10);
        let _p = curve.point(0, 0);
    }
}
