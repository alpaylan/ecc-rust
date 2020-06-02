use core::ops::Add;
use core::ops::Sub;
use core::ops::Neg;
use core::ops::Mul;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point {
    curve: EllipticCurve,
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        let point_zero = Point{curve: self.curve, x: 0, y: 0};
        if self ==  point_zero {
            return other;
        } else if other == point_zero {
            return self;
        } else {
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
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, scalar: i32) -> Self::Output {
        // TODO: Make this double_and_add
        let mut i = 0;
        let mut p = Point{
                        curve:self.curve,
                        x: 0, 
                        y: 0
                        };
        while i < scalar {
            p = p + self;
            i += 1;
        }
        return p;
    }
}
impl Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        // TODO: Make this double_and_add
        let mut i = 0;
        let mut p = Point{
                        curve:point.curve,
                        x: 0, 
                        y: 0
                        };
        println!("Scalar is {}", self);
        while i < self {
            p = p + point;
            i += 1;
        }
        return p;
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
    fn mul() {
        let curve = EllipticCurve::new(-7, 10);

        let p = curve.point(1, 2);
        
        assert_eq!(p, 1*p);
        assert_eq!(p + p, 2*p);
        assert_eq!(p + p, p*2);
    }
    #[test]
    #[should_panic]
    fn invalid_point() {
        let curve = EllipticCurve::new(-7, 10);
        let _p = curve.point(0, 0);
    }
}

// Burda mısın