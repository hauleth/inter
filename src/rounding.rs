use libc;
use std::num::FromPrimitive;

extern {
    fn fesetround(flag: libc::c_int) -> libc::c_int;
    fn fegetround() -> libc::c_int;
}

#[derive(FromPrimitive)]
pub enum Rounding {
    ToNearest  = 0x0000,
    Downward   = 0x0400,
    Upward     = 0x0800,
    TowardZero = 0x0C00,
}

impl Rounding {
    pub fn current() -> Self {
        FromPrimitive::from_i32(unsafe { fegetround() }).unwrap()
    }

    pub fn set(self) {
        unsafe { fesetround(self as libc::c_int) };
    }

    pub fn execute<R, T: FnOnce() -> R>(self, func: T) -> R {
        let old = unsafe { fesetround(self as libc::c_int) };
        let ret = func();
        unsafe { fesetround(old) };

        ret
    }
}
