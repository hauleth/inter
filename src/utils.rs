pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}
