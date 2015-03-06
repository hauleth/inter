//! Interval arithmetic for Rust.
//!
//! Created as part of Numerical Analysis at Computer Engineering classes at PUT

#![feature(core)]
#![feature(libc)]

extern crate num;
extern crate libc;

pub mod interval;
pub mod rounding;

pub use interval::Interval;
