extern crate inter;

use inter::Interval;
use std::num::{
    Float
};

fn main() {
    let x = 0.785;
    let int = Interval::with_epsilon(x, 0.02);
    let isin = int.sin();
    println!("{}", x.sin());
    println!("{} {}", isin, isin.width());
}
