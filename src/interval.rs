use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg
};
use std::fmt;
use std::num::{
    Float,
    cast,
    NumCast
};
use std::cmp::{
    Ordering,
    partial_min,
    partial_max
};
use num::{
    Zero,
    One,
    Num,
};
use super::rounding::Rounding;

/// Range arithmetic structure
///
/// ## Examples
///
/// ```rust
/// use inter::Interval;
///
/// let a = Interval::with_range(1., 2.);
/// let b = Interval::with_range(1.5, 2.5);
///
/// let sum = a + b;
///
/// println!("{}", sum);
/// ```
///
/// ## Quirks
/// - this implement `PartialEq` for type `T`
///
///   ```rust
///   use inter::Interval;
///
///   assert!(Interval::with_range(1., 2.) == 1.5);
///   ```
///
///   which may be strange when we see that `PartialEq` require to `a == b && b == c && a == c`,
///   but when we accept that this is *interval arithmetic* then this is all true within
///   `Interval::epsilon()`.
/// - this implement `PartialOrd` for type `T`
///
///   ```rust
///   use inter::Interval;
///
///   assert!(Interval::with_range(1., 2.) < 3.);
///   assert!(Interval::with_range(1., 2.) > 0.5);
///   assert!(Interval::with_range(1., 2.) <= 1.5);
///   assert!(Interval::with_range(1., 2.) >= 1.5);
///   ```
#[derive(Copy, Debug, PartialEq)]
pub struct Interval<T> {
    start: T,
    end: T
}

impl<T> Interval<T>
where T: PartialOrd {
    /// Create interval with start and end of range
    ///
    /// # Panics
    ///
    /// This will panic if `start` is greater than `end`. Only proper intervals are allowed.
    pub fn with_range(start: T, end: T) -> Self {
        assert!(start <= end);

        Interval {
            start: start,
            end: end
        }
    }

    /// Create interval with central element and deviation ε
    pub fn with_epsilon<P>(center: P, epsilon: P) -> Self
        where P: Add<Output = T> + Sub<Output = T> + Copy {
            Interval::with_range(center - epsilon, center + epsilon)
        }

    pub fn exact(value: T) -> Self
    where T: Num + Copy {
        Interval::with_epsilon(value, Zero::zero())
    }
}

impl<T> Zero for Interval<T>
where T: Num + Copy + PartialOrd {
    fn zero() -> Self {
        Interval::exact(Zero::zero())
    }

    fn is_zero(&self) -> bool {
        self.start == Zero::zero() && self.end == Zero::zero()
    }
}

impl<T> One for Interval<T>
where T: Num + Copy + PartialOrd {
    fn one() -> Self {
        Interval::exact(One::one())
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

    /// Calculate intersection of intervals.
    ///
    /// ## Returns
    ///
    /// - `None` if there is no overlap
    /// - `Some(Interval)` otherwise
    ///
    /// ## Example
    ///
    /// ```rust
    /// use inter::Interval;
    /// let a = Interval::with_range(1., 2.);
    /// let b = Interval::with_range(1.5, 2.5);
    /// let c = Interval::with_range(3., 4.);
    ///
    /// assert_eq!(a.intersection(&b), b.intersection(&a));
    /// assert_eq!(a.intersection(&b), Some(Interval::with_range(1.5, 2.)));
    /// assert_eq!(a.intersection(&c), None);
    /// ```
    pub fn intersection(&self, other: &Interval<T>) -> Option<Interval<T>>
        where T: PartialOrd {
            let low = partial_max(self.start, other.start);
            let high = partial_min(self.end, other.end);

            if low.is_none() || high.is_none() || low > high {
                return None
            }

            Some(Interval {
                start: low.unwrap(),
                end: high.unwrap()
            })
        }

    /// Return ε (half of interval width)
    ///
    /// ## Example
    ///
    /// ```rust
    /// use inter::Interval;
    /// let interval = Interval::with_range(1., 2.);
    ///
    /// assert_eq!(interval.epsilon(), 0.5);
    /// ```
    pub fn epsilon(&self) -> T
        where T: Sub<Output = T> + Div<Output = T> + NumCast {
            self.width() / cast(2).unwrap()
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
        let start = Rounding::Downward.execute(|| self.start + other.start);
        let end = Rounding::Upward.execute(|| self.end + other.end);
        Interval {
            start: start,
            end: end
        }
    }
}

impl<T> Sub for Interval<T>
where T: Sub<Output = T> + Copy {
    type Output = Interval<T>;

    fn sub(self, other: Self) -> Self {
        let start = Rounding::Downward.execute(|| self.start - other.start);
        let end = Rounding::Upward.execute(|| self.end - other.end);
        Interval {
            start: start,
            end: end
        }
    }
}

impl<T> Mul for Interval<T>
where T: Mul<Output = T> + Copy + PartialOrd {
    type Output = Interval<T>;

    fn mul(self, other: Self) -> Self {
        let (a, b, c, d) = (self.start, self.end, other.start, other.end);
        let min = Rounding::Downward.execute(|| {
            vec![a * d, b * c, b * d].into_iter().fold(a * c, |acc, i| partial_min(acc, i).unwrap())
        });
        let max = Rounding::Upward.execute(|| {
            vec![a * d, b * c, b * d].into_iter().fold(a * c, |acc, i| partial_max(acc, i).unwrap())
        });

        Interval {
            start: min,
            end: max
        }
    }
}

impl<T> Div for Interval<T>
where T: Div<Output = T> + Copy + PartialOrd {
    type Output = Interval<T>;

    fn div(self, other: Self) -> Self {
        let (a, b, c, d) = (self.start, self.end, other.start, other.end);
        let min = Rounding::Downward.execute(|| {
            vec![a / d, b / c, b / d].into_iter().fold(a / c, |acc, i| partial_min(acc, i).unwrap())
        });
        let max = Rounding::Upward.execute(|| {
            vec![a / d, b / c, b / d].into_iter().fold(a / c, |acc, i| partial_max(acc, i).unwrap())
        });

        Interval {
            start: min,
            end: max
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

impl<T> Interval<T>
where T: Float + Num {
    pub fn sin(self) -> Self {
        let x2 = self * self;

        let mut ret = (1..500_000).fold(self, |acc, i| {
            let mul: T = cast(2*i * (2*i + 1)).unwrap();
            let int = Interval::exact(mul);
            let mul = x2 / int;
            if i % 2 == 0 { acc * mul + acc } else { acc * mul - acc }
        });

        ret.start = ret.start.max(cast(-1).unwrap()).min(One::one());
        ret.end = ret.end.max(cast(-1).unwrap()).min(One::one());

        ret
    }

    // pub fn cos(self) -> Self {
    //     let x2 = self * self;

    //     let mut ret = (1..500_000).fold(One::one(), |acc, i| {
    //         let mul: T = cast(2*i * (2*i + 1)).unwrap();
    //         let int = Interval::exact(mul);
    //         let mul = x2 / int;
    //         if i % 2 == 0 { acc * mul + acc } else { acc * mul - acc }
    //     });

    //     ret.start = ret.start.max(cast(-1).unwrap()).min(One::one());
    //     ret.end = ret.end.max(cast(-1).unwrap()).min(One::one());

    //     ret
    // }
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
