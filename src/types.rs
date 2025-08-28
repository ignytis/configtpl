use std::{ffi::c_char, ptr::null};

use minijinja::Error;

use crate::ffi::utils::strings::string_to_cchar;


/// Handle allocated for initialized Jinja environment.
/// All operations which invoke the Jinja envionment should have this handle provided
pub type EnrironmentHandle = usize;

/**
 * @brief Indicates the place where error occurred.
 * 
 * This structure is copied from inja lib (SourceLocation) in order not to depend on it
 */
#[repr(C)]
#[derive(Default)]
pub struct TemplateErrorLocation {
    line: usize,
    start: usize,
    end: usize,
}

/**
 * @brief Status of template rendering
 */
#[repr(C)]
#[derive(Default)]
pub enum RenderStatus
{
    RenderStatusSuccess = 0,
    RenderStatusErrorInvalidHandle = 1,
    RenderStatusErrorTemplateRender = 2,
    #[default]
    RenderStatusErrorUnknown = 255,
}

/**
 * @brief Result of template rendering
 */
#[repr(C)]
#[derive(Default)]
pub struct RenderResult
{
    pub status: RenderStatus,
    pub output: *const c_char,
    pub location: TemplateErrorLocation,
}

impl RenderResult {
    pub fn new_invalid_handle() -> Self {
        Self {
            status: RenderStatus::RenderStatusErrorInvalidHandle,
            output: null(),
            location: TemplateErrorLocation::default(),
        }
    }
}

impl From<String> for RenderResult {
    fn from(value: String) -> Self {
        Self {
            status: RenderStatus::RenderStatusSuccess,
            output: string_to_cchar(value),
            location: Default::default(),
        }
    }
}

impl From<Error> for RenderResult {
    fn from(value: Error) -> Self {
        Self {
            status: RenderStatus::RenderStatusErrorTemplateRender,
            output: string_to_cchar(value.to_string()),
            location: TemplateErrorLocation {
                line: value.line().unwrap(),
                start: value.range().unwrap().start,
                end: value.range().unwrap().end,
            },
        }
    }
}