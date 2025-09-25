use crate::shared_lib::ffi::types::std_types;

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