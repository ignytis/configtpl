use libc::{c_char, c_uint, c_ushort, size_t, c_long};

pub type Bool = c_ushort;
pub type Char = c_char;
pub type LongInt = c_long;
pub type UInt = c_uint;
pub type USize = size_t;

pub type ConstBytePtr = *const u8;
pub type ConstCharPtr = *const c_char;
pub type MutCharPtr = *mut c_char;
