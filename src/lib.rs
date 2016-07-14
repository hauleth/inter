//! Interval arithmetic for Rust.
//!
//! Created as part of Numerical Analysis at Computer Engineering classes at PUT

extern crate num_traits as num;
extern crate libc;

mod utils;
pub mod interval;
pub mod rounding;

pub use interval::Interval;
