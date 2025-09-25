use std::ptr::null;

use minijinja::Error;

use crate::shared_lib::ffi::{
    types::std_types::{ConstCharPtr, UInt, USize},
    utils::strings::string_to_cchar
};


/// Handle allocated for initialized Jinja environment.
/// All operations which invoke the Jinja envionment should have this handle provided
pub type EnrironmentHandle = UInt;

/**
 * @brief Indicates the place where error occurred.
 * 
 * This structure is copied from inja lib (SourceLocation) in order not to depend on it
 */
#[repr(C)]
#[derive(Default)]
pub struct TemplateErrorLocation {
    line: USize,
    start: USize,
    end: USize,
}

/**
 * @brief Status of template rendering
 */
#[repr(C)]
#[derive(Default)]
pub enum RenderStatus
{
    Success = 0,
    ErrorInvalidHandle = 1,
    ErrorTemplateRender = 2,
    #[default]
    ErrorUnknown = 255,
}

/**
 * @brief Result of template rendering
 */
#[repr(C)]
#[derive(Default)]
pub struct RenderResult
{
    pub status: RenderStatus,
    pub output: ConstCharPtr,
    pub location: TemplateErrorLocation,
}

impl RenderResult {
    pub fn new_invalid_handle() -> Self {
        Self {
            status: RenderStatus::ErrorInvalidHandle,
            output: null(),
            location: TemplateErrorLocation::default(),
        }
    }
}

impl From<String> for RenderResult {
    fn from(value: String) -> Self {
        Self {
            status: RenderStatus::Success,
            output: string_to_cchar(value),
            location: Default::default(),
        }
    }
}

impl From<Error> for RenderResult {
    fn from(value: Error) -> Self {
        Self {
            status: RenderStatus::ErrorTemplateRender,
            output: string_to_cchar(value.to_string()),
            location: TemplateErrorLocation {
                line: value.line().unwrap(),
                start: value.range().unwrap().start,
                end: value.range().unwrap().end,
            },
        }
    }
}