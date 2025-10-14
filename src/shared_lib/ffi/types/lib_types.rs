use crate::{shared_lib::ffi::
    {
        types::{
            config_param::ConfigParam,
            std_types::{ConstCharPtr, UInt}
        },
        utils::strings::string_to_cchar
    },
    types::config_param::ConfigParam as LibConfigParam
};


/// Handle allocated for initialized Config Builder.
/// All operations which invoke the Config Builder should have this handle provided
pub type CfgBuilderHandle = UInt;

/// Simple result of function call. Used for function which don't (yet?) need to return more complex result.
#[repr(C)]
pub enum SimpleResult
{
    Success = 0,
    Error = 1,
}

/// Status of configuration building
#[repr(C)]
#[derive(Default)]
pub enum BuildStatus
{
    Success = 0,
    /// An invalid handle is provided
    ErrorInvalidHandle = 1,
    /// Indicates that an error occurred during building the config
    ErrorBuilding = 200,
    #[default]
    /// An unknown error. Should not occur in general.
    ErrorUnknown = 255,
}

/// Result of configuration building
#[repr(C)]
#[derive(Default)]
pub struct BuildResult
{
    pub status: BuildStatus,
    pub output: ConfigParam,
    pub error_msg: ConstCharPtr,
}

impl BuildResult {
    pub fn new_error_invalid_handle() -> Self {
        Self {
            status: BuildStatus::ErrorInvalidHandle,
            output: ConfigParam::new_null(),
            error_msg: Default::default(),
        }
    }

    pub fn new_error_building(msg: &String) -> Self {
        Self {
            status: BuildStatus::ErrorBuilding,
            output: ConfigParam::new_null(),
            error_msg: string_to_cchar(msg),
        }
    }
}

/// TODO: delete? We can probably return only *const-s
impl From<LibConfigParam> for BuildResult {
    fn from(value: LibConfigParam) -> Self {
        Self {
            status: BuildStatus::Success,
            output: value.into(),
            error_msg: Default::default(),
        }
    }
}

impl From<LibConfigParam> for *const BuildResult {
    /// NB! Leaks the value
    fn from(value: LibConfigParam) -> Self {
        let r: BuildResult = value.into();
        Box::into_raw(Box::new(r))
    }
}

impl Into<*const BuildResult> for BuildResult {
    /// NB! Leaks the value
    fn into(self) -> *const BuildResult {
        Box::into_raw(Box::new(self))
    }
}
