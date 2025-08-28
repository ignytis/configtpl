use libc::{c_char, c_uint, size_t};

pub type Char = c_char;
pub type UInt = c_uint;
pub type USize = size_t;

pub type ConstBytePtr = *const u8;
pub type ConstCharPtr = *const c_char;
pub type ConstCStrPtr = *const i8;
pub type MutCharPtr = *mut c_char;
