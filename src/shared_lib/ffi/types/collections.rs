use std::collections::HashMap;

use crate::{shared_lib::ffi::{types::std_types, utils::strings::{cchar_to_string, string_to_cchar}}, types::config_param::ConfigParam};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Array<T> {
    pub data: *mut T,
    pub len: std_types::UInt,
}

impl<T> Array<T> {
    pub fn new_of_len(len: usize) -> Array<T> {
        let mut data: Vec<T> = Vec::with_capacity(len);
        Array {
            data: unsafe { std::slice::from_raw_parts_mut(data.as_mut_ptr(), std::mem::size_of::<T>() * len).as_mut_ptr() },
            len: len as std_types::UInt,
        }
    }

    /// NB! Leaks memory. You need to call `free_contents` to deallocate memory.
    pub fn from_vec(vector: Vec<T>) -> Array<T> {
        let len = vector.len() as std_types::UInt;
        let mut boxed_slice: Box<[T]> = vector.into_boxed_slice();
        let arr: Array<T> = Array {
            data: boxed_slice.as_mut_ptr(),
            len,
        };
        std::mem::forget(boxed_slice);
        arr
    }

    /// Deallocates memory for array
    pub fn free_contents(&mut self) {
        let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.len as usize) };
        let s = s.as_mut_ptr();
        unsafe {
            let _ = Box::from_raw(s);
        }
        self.len = 0;
    }
}

impl<T> Default for Array<T> {
    fn default() -> Self {
        Array::new_of_len(0)
    }
}

/// Array of two strings
pub type StringKV = [std_types::ConstCharPtr; 2];
/// Array of key-value pairs
pub type ArrayStringKV = Array<StringKV>;

/// TODO: move to ConfigParam::into? ConfigParam is less abstract type than Array
impl Into<ConfigParam> for *const ArrayStringKV {
    fn into(self) -> ConfigParam {
        let ptr = unsafe { self.read() };
        let mut config_param: HashMap<String, ConfigParam> = HashMap::new();
        for i in 0..ptr.len {
            let item  = unsafe { ptr.data.offset(i as isize)  };
            let k = cchar_to_string(item.wrapping_add(0) as *const i8);
            let v = cchar_to_string(item.wrapping_add(1) as *const i8);
            config_param.insert(k, ConfigParam::String(v));
        }
        ConfigParam::HashMap(config_param)
    }
}

/// TODO: move to ConfigParam::into? ConfigParam is less abstract type than Array
impl From<ConfigParam> for ArrayStringKV {
    fn from(param: ConfigParam) -> Self {
        let data = match param {
            ConfigParam::HashMap(m) => m,
            _ => HashMap::default(),
        };
        let flat_data = flatten_config_hashmap(&data);
        let result = ArrayStringKV::new_of_len(flat_data.len());
        for (i, (k, v)) in flat_data.iter().enumerate() {
            let item = unsafe { result.data.offset(i as isize) };

            unsafe { *item = [string_to_cchar(k), string_to_cchar(v)] };
        }

        result
    }
}

/// Flattens the config param.
/// TODO: get rid ot this function? Flat KV maps will be replaced with structures
fn flatten_config_hashmap(hm: &HashMap<String, ConfigParam>) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();

    for (k, v) in hm {
        match v {
            ConfigParam::Boolean(b) => {
                result.insert(k.clone(), b.to_string());
            },
            ConfigParam::Float(n) => {
                result.insert(k.clone(), n.to_string());
            },
            ConfigParam::Int(n) => {
                result.insert(k.clone(), n.to_string());
            },
            ConfigParam::HashMap(hm2) => {
                for (k2, v2) in flatten_config_hashmap(hm2) {
                    result.insert(format!("{}.{}", k, k2), v2);
                }
            },
            ConfigParam::Null => {
                result.insert(k.clone(), "null".to_string());
            }
            ConfigParam::String(s) => {
                result.insert(k.clone(), s.clone());
            },
            ConfigParam::Vec(_a) => {
                // TODO: implement vec flattening
            }
        }
    }

    result
}
