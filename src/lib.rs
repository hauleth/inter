use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg
};
use std::fmt;

#[derive(Copy, Debug, PartialEq, PartialOrd)]
pub struct Interval<T>
where T: Copy {
    start: T,
    end: T
}

impl<T> Interval<T>
where T: Copy + PartialOrd + fmt::Debug {
    pub fn with_range(start: T, end: T) -> Self {
        assert!(start <= end, "{:?} must be no greater than {:?}", start, end);

        Interval {
            start: start,
            end: end
        }
    }

    pub fn with_epsilon<P>(center: P, epsilon: P) -> Self
    where P: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        assert!(epsilon < center);
        Interval::with_range(center - epsilon, center + epsilon)
    }

    pub fn contains(&self, value: T) -> bool {
        self.start < value && value < self.end
    }

    pub fn width(&self) -> T
    where T: Sub<Output = T> {
        self.end - self.start
    }
}

impl<T> fmt::Display for Interval<T>
where T: fmt::Display + Copy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}, {}]", self.start, self.end)
    }
}

impl<T> Add for Interval<T>
where T: Add<Output = T> + Copy {
    type Output = Interval<T>;

    fn add(self, other: Self) -> Self {
        Interval {
            start: self.start + other.start,
            end: self.end + other.end
        }
    }
}

impl<T> Sub for Interval<T>
where T: Sub<Output = T> + Copy {
    type Output = Interval<T>;

    fn sub(self, other: Self) -> Self {
        Interval {
            start: self.start - other.start,
            end: self.end - other.end
        }
    }
}

impl<T> Mul for Interval<T>
where T: Mul<Output = T> + Copy + PartialOrd {
    type Output = Interval<T>;

    fn mul(self, other: Self) -> Self {
        let (a, b, c, d) = (self.start, self.end, other.start, other.end);
        let mut v = vec![a * c, a * d, b * c, b * d];
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Interval {
            start: v[0],
            end: v[3]
        }
    }
}

impl<T> Div for Interval<T>
where T: Div<Output = T> + Copy + PartialOrd {
    type Output = Interval<T>;

    fn div(self, other: Self) -> Self {
        let (a, b, c, d) = (self.start, self.end, other.start, other.end);
        let mut v = vec![a / c, a / d, b / c, b / d];
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Interval {
            start: v[0],
            end: v[3]
        }
    }
}

impl<T> Neg for Interval<T>
where T: Neg<Output = T> + Copy {
    type Output = Interval<T>;

    fn neg(self) -> Self {
        Interval {
            start: -self.end,
            end: -self.start
        }
    }
}

#[cfg(test)]
mod test {
    use super::Interval;

    fn setup() -> (Interval<f64>, Interval<f64>) {
        (Interval::with_range(1., 2.), Interval::with_range(3., 4.))
    }

    #[test]
    fn contains() {
        let (a, _) = setup();
        assert!(a.contains(1.5));
    }

    #[test]
    fn width() {
        let (a, _) = setup();
        assert_eq!(a.width(), 1.);
    }

    #[test]
    fn display() {
        let (a, _) = setup();
        assert_eq!(format!("{}", a), "[1, 2]".to_string());
    }

    #[test]
    fn addition() {
        let (a, b) = setup();
        assert_eq!(a + b, Interval::with_range(4., 6.));
    }

    #[test]
    fn substraction() {
        let (a, b) = setup();
        assert_eq!(b - a, Interval::with_range(2., 2.));
    }

    #[test]
    fn multiply() {
        let (a, b) = setup();
        assert_eq!(a * b, Interval::with_range(3., 8.));
    }

    #[test]
    fn divide() {
        let (a, b) = setup();
        assert_eq!(b / a, Interval::with_range(1.5, 4.));
    }

    #[test]
    fn negate() {
        let (a, _) = setup();
        assert_eq!(-a, Interval::with_range(-2., -1.));
    }
}
