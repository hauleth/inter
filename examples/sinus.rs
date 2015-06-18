extern crate inter;
extern crate num;

use inter::Interval;
use num::Float;

fn main() {
    let x = 0.785;
    let int = Interval::with_epsilon(x, 0.02);
    let isin = int.sin();
    println!("{}", x.sin());
    println!("{} {}", isin, isin.width());
}
