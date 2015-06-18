extern crate inter;
extern crate num;

use inter::rounding::Rounding;
use num::{
    Float,
    FromPrimitive
};

fn calc<T>(name: &str)
where T: Float + FromPrimitive + std::fmt::LowerExp {
    let x: T = FromPrimitive::from_f64(1.0).unwrap();
    let y: T = FromPrimitive::from_f64(1.0e-20).unwrap();
    let z1 = x - y;
    let z2 = y - x;
    let z1 = z1 - x;
    let z2 = z2 + x;
    println!("{:12}, z1 = {:17.10e}, z2 = {:17.10e}", name, z1, z2);
}
fn main() {
    println!("f32");
    Rounding::ToNearest.execute(|| calc::<f32>("nearest"));
    Rounding::Downward.execute(|| calc::<f32>("downward"));
    Rounding::Upward.execute(|| calc::<f32>("upward"));
    Rounding::TowardZero.execute(|| calc::<f32>("toward zero"));
    println!("\nf64");
    Rounding::ToNearest.execute(|| calc::<f64>("nearest"));
    Rounding::Downward.execute(|| calc::<f64>("downward"));
    Rounding::Upward.execute(|| calc::<f64>("upward"));
    Rounding::TowardZero.execute(|| calc::<f64>("toward zero"));
}
