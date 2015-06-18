use libc::c_int;
use num::FromPrimitive;

extern {
    fn fesetround(flag: c_int) -> c_int;
    fn fegetround() -> c_int;
}

pub enum Rounding {
    ToNearest  = 0x0000,
    Downward   = 0x0400,
    Upward     = 0x0800,
    TowardZero = 0x0C00,
}

impl FromPrimitive for Rounding {
    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0x0000 => Some(Rounding::ToNearest ),
            0x0400 => Some(Rounding::Downward  ),
            0x0800 => Some(Rounding::Upward    ),
            0x0C00 => Some(Rounding::TowardZero),
            _ => None
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        FromPrimitive::from_i64(n as i64)
    }
}

impl Rounding {
    pub fn current() -> Option<Self> {
        FromPrimitive::from_i32(unsafe { fegetround() })
    }

    pub fn set(self) -> Result<(), ()> {
        let res = unsafe { fesetround(self as c_int) };
        if res == 0 {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn execute<R, T: FnOnce() -> R>(self, func: T) -> R {
        let old = unsafe { fesetround(self as c_int) };
        let ret = func();
        unsafe { fesetround(old) };

        ret
    }
}
