use std::collections::HashMap;

use crate::{
    shared_lib::ffi::{types::{
        collections::Array,
        std_types::{Bool, ConstCharPtr, LongInt}
    }, utils::strings::{cchar_const_deallocate, cchar_to_string, string_to_cchar}},
    types::config_param::ConfigParam as LibConfigParam
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum ConfigParamType {
    Boolean,
    Map,
    Int,
    Null,
    String,
    Vec,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union ConfigParamValue {
    pub boolean: Bool,
    pub map: Array<ConfigParamDictItem>,
    pub integer: LongInt,
    pub null: (),
    pub string: ConstCharPtr,
    pub vector: Array<ConfigParam>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConfigParam {
    pub param_type: ConfigParamType,
    pub value: ConfigParamValue,
}

impl ConfigParam {
    pub fn new_bool(v: bool) -> Self {
        Self {
            param_type: ConfigParamType::Boolean,
            value: ConfigParamValue{ boolean: v as Bool },
        }
    }

    pub fn new_int(v: LongInt) -> Self {
        Self {
            param_type: ConfigParamType::Int,
            value: ConfigParamValue{ integer: v },
        }
    }

    pub fn new_map(c: &HashMap<String, LibConfigParam>) -> Self {
        let hm: Array<ConfigParamDictItem> = Array::from_vec(
            c.iter()
                .map(|(k, v)| ConfigParamDictItem::new_from_string_kv(k, v))
                .collect()
        );

        Self {
            param_type: ConfigParamType::Map,
            value: ConfigParamValue{ map: hm },
        }
    }

    pub fn new_null() -> Self {
        Self {
            param_type: ConfigParamType::Null,
            value: ConfigParamValue{ null: () },
        }
    }

    pub fn new_string<S: Into<String>>(s: S) -> Self {
        Self {
            param_type: ConfigParamType::String,
            value: ConfigParamValue{ string: string_to_cchar(s) },
        }
    }

    pub fn new_vec(c: &Vec<LibConfigParam>) -> Self {
        let vect: Array<ConfigParam> = Array::new_of_len(c.len());
        for (i, item_src) in c.into_iter().enumerate() {
            let item_dst = unsafe { vect.data.offset(i as isize) };
            unsafe { *item_dst = item_src.clone().into() };
        }
        Self {
            param_type: ConfigParamType::Vec,
            value: ConfigParamValue{ vector: vect },
        }
    }

    /// Deallocates memory for stored data
    pub fn free_contents(&mut self) {
        match self.param_type {
            ConfigParamType::Map => {
                let mut c = unsafe { self.value.map };
                for i in 0..c.len {
                    let item: *mut ConfigParamDictItem = unsafe { c.data.offset(i as isize) };
                    unsafe { (*item).free_contents() };
                }
                c.free_contents();
            },
            ConfigParamType::Vec => {
                let mut c = unsafe { self.value.vector };
                for i in 0..c.len {
                    let item: *mut ConfigParam = unsafe { c.data.offset(i as isize) };
                    unsafe { (*item).free_contents() };
                }
                c.free_contents();
            },
            ConfigParamType::String => {
                let c = unsafe { self.value.string };
                cchar_const_deallocate(c);
            }
            _ => {}, // no deallocation needed for scalar types
        }
    }

    /// Debug printing the config param
    pub fn debug_print(&self, prefix: Option<String>) {
        let prefix = prefix.unwrap_or_default();
        unsafe {
            match self.param_type {
                ConfigParamType::Boolean => println!("{}: {}", prefix, self.value.boolean),
                ConfigParamType::Map => {
                    for i in 0..self.value.map.len {
                        let item = self.value.map.data.offset(i as isize);
                        (*(*item).value).debug_print(Some(format!("{}.{}", prefix, cchar_to_string((*item).name))));
                    }
                },
                ConfigParamType::Int => println!("{}: {}", prefix, self.value.integer),
                ConfigParamType::Null => println!("{}: null", prefix),
                ConfigParamType::String => println!("{}: {}", prefix, cchar_to_string(self.value.string)),
                ConfigParamType::Vec => {
                    for i in 0..self.value.vector.len {
                        let item = *self.value.vector.data.offset(i as isize);
                        item.debug_print(Some(format!("{}[{}]", prefix, i)));
                    }
                },
            }
        }
    }
}

impl Default for ConfigParam {
    fn default() -> Self {
        Self::new_null()
    }
}

impl From<LibConfigParam> for ConfigParam {
    fn from(param: LibConfigParam) -> Self {
        ConfigParam::from(&param)
    }
}

impl From<&LibConfigParam> for ConfigParam {
    fn from(param: &LibConfigParam) -> Self {
        match param {
            LibConfigParam::Boolean(v) => Self::new_bool(*v),
            LibConfigParam::HashMap(v) => Self::new_map(&v),
            LibConfigParam::Int(v) => Self::new_int(*v),
            LibConfigParam::Null => Self::new_null(),
            LibConfigParam::String(v) => Self::new_string(v),
            LibConfigParam::Vec(v) => Self::new_vec(&v),
        }
    }
}

impl From<&LibConfigParam> for *const ConfigParam {
    /// NB! Leaks memory. Need to deallocate the pointer to ConfigParam manually.
    fn from(param: &LibConfigParam) -> Self {
        let config_param = ConfigParam::from(param);
        Box::into_raw(Box::new(config_param))
    }
}

impl Into<LibConfigParam> for ConfigParam {
    fn into(self) -> LibConfigParam {
        match self.param_type {
            ConfigParamType::Boolean => LibConfigParam::Boolean(unsafe { self.value.boolean > 0 }),
            ConfigParamType::Map => {
                let mut map: HashMap<String, LibConfigParam> = HashMap::new();
                unsafe {
                    for i in 0..self.value.map.len {
                        let src_obj = self.value.map.data.offset(i as isize);
                        map.insert(cchar_to_string((*src_obj).name), (*(*src_obj).value).into());
                    }
                }
                LibConfigParam::HashMap(map)
            },
            ConfigParamType::Int => LibConfigParam::Int(unsafe { self.value.integer }),
            ConfigParamType::Null => LibConfigParam::Null,
            ConfigParamType::String => LibConfigParam::String(unsafe { cchar_to_string(self.value.string) }),
            ConfigParamType::Vec => {
                let mut vec: Vec<LibConfigParam> = Vec::new();
                unsafe {
                    for i in 0..self.value.map.len {
                        let src_obj = self.value.map.data.offset(i as isize);
                        vec.push((*(*src_obj).value).into());
                    }
                }
                LibConfigParam::Vec(vec)
            },
        }
    }
}

/// An item in configuration param of dicrionary type
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ConfigParamDictItem {
    pub name: ConstCharPtr,
    pub value: *const ConfigParam,
}

impl ConfigParamDictItem {
    /// NB! Leaks memory. Need to deallocate the pointer to ConfigParam manually.
    pub fn new_from_string_kv<S: Into<String>>(name: S, value: &LibConfigParam) -> Self {
        let name = string_to_cchar(name);
        let value: *const ConfigParam = value.into();
        Self { name , value }
    }

    pub fn free_contents(&mut self) {
        unsafe {
            let mut v = Box::from_raw(self.value.cast_mut());
            v.free_contents();
            cchar_const_deallocate(self.name);
        }
    }
}
