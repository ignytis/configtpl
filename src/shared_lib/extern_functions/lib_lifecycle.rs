use crate::shared_lib::ffi::types::lib_types::SimpleResult;

/// This function should be invoked before any other library routines.
/// Right at the moment it does nothing, but some initialization might be added in the future.
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_init() -> SimpleResult {
    SimpleResult::Success
}


/// This function should be invoked after any other library routines.
/// /// Right at the moment it does nothing, but some final actions might be added in the future.
#[unsafe(no_mangle)]
pub extern "C" fn configtpl_cleanup() -> SimpleResult {
    SimpleResult::Success
}
