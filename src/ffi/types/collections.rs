#[repr(C)]
#[derive(Clone, Copy)]
pub struct LinkedList<T> {
    pub item: *mut T,
    pub next: *const LinkedList<T>,
}