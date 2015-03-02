//! Interval arithmetic for Rust.
//!
//! Created as part of Numerical Analysis at Computer Engineering classes at PUT

#![feature(core)]

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg
};
use std::fmt;
use std::num::{
    cast,
    NumCast
};
use std::cmp::Ordering;

/// Range arithmetic structure
#[derive(Copy, Debug, PartialEq)]
pub struct Interval<T>
where T: Copy {
    start: T,
    end: T
}

impl<T> Interval<T>
where T: Copy + PartialOrd + fmt::Debug {
    /// Create interval with start and end of range
    ///
    /// This will panic if `start` is greater than `end`. Only proper intervals are allowed.
    pub fn with_range(start: T, end: T) -> Self {
        assert!(start <= end, "{:?} must be no greater than {:?}", start, end);

        Interval {
            start: start,
            end: end
        }
    }

    /// Create interval with central element and deviation ε
    pub fn with_epsilon<P>(center: P, epsilon: P) -> Self
    where P: Add<Output = T> + Sub<Output = T> + PartialOrd + Copy {
        Interval::with_range(center - epsilon, center + epsilon)
    }
}

impl<T> Interval<T>
where T: Copy {
    /// Check if value fit inside range
    ///
    /// ## Example
    ///
    /// ```rust
    /// use inter::Interval;
    /// let interval = Interval::with_range(1., 2.);
    ///
    /// assert!(interval.contains(1.5));
    /// assert!(!interval.contains(2.1))
    /// ```
    pub fn contains(&self, value: T) -> bool
    where T: PartialOrd {
        self.start <= value && value <= self.end
    }

    /// Width of interval
    ///
    /// ## Example
    ///
    /// ```rust
    /// use inter::Interval;
    /// let interval = Interval::with_range(1., 2.);
    ///
    /// assert_eq!(interval.width(), 1.);
    /// ```
    pub fn width(&self) -> T
    where T: Sub<Output = T> {
        self.end - self.start
    }

    /// Central element of interval (mean)
    ///
    /// ## Example
    ///
    /// ```rust
    /// use inter::Interval;
    /// let interval = Interval::with_range(1., 2.);
    ///
    /// assert_eq!(interval.center(), 1.5);
    /// ```
    pub fn center(&self) -> T
    where T: Add<Output = T> + Div<Output = T> + NumCast {
        (self.start + self.end) / cast(2).unwrap()
    }
}

impl<T> fmt::Display for Interval<T>
where T: fmt::Display + Copy {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}, {}]", self.start, self.end)
    }
}

impl<T> PartialEq<T> for Interval<T>
where T: PartialOrd + Copy {
    fn eq(&self, other: &T) -> bool {
        self.contains(*other)
    }
}

impl<T> PartialOrd<T> for Interval<T>
where T: PartialOrd + Copy {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        if *other < self.start {
            return Some(Ordering::Greater)
        }
        if *other > self.end {
            return Some(Ordering::Less)
        }
        if self.contains(*other) {
            return Some(Ordering::Equal)
        }

        None
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
        assert!(!a.contains(2.1));
        assert!(a == 1.5);
        assert!(a != 2.1);
    }

    #[test]
    fn ordering() {
        let (a, _) = setup();
        assert!(a > 0.);
        assert!(a < 3.);
        assert!(a <= 1.5);
        assert!(a >= 1.5);
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
