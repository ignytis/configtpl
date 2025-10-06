use crate::{shared_lib::ffi::{types::{
        collections::ArrayStringKV,
        std_types::{ConstCharPtr, UInt},
    }, utils::strings::string_to_cchar}, types::config_param::ConfigParam};


/// Handle allocated for initialized Config Builder.
/// All operations which invoke the Config Builder should have this handle provided
pub type CfgBuilderHandle = UInt;

/**
 * @brief Status of configuration building
 */
#[repr(C)]
#[derive(Default)]
pub enum BuildStatus
{
    Success = 0,
    ErrorInvalidHandle = 1,
    /// Indicates that an error occurred during building the config
    ErrorBuilding = 200,
    #[default]
    /// An unknown error. Should not occur in general.
    ErrorUnknown = 255,
}

/**
 * @brief Result of configuration building
 */
#[repr(C)]
#[derive(Default)]
pub struct BuildResult
{
    pub status: BuildStatus,
    pub output: ArrayStringKV,
    pub error_msg: ConstCharPtr,
}

impl BuildResult {
    pub fn new_error_invalid_handle() -> Self {
        Self {
            status: BuildStatus::ErrorInvalidHandle,
            output: ArrayStringKV::default(),
            error_msg: Default::default(),
        }
    }

    pub fn new_error_building(msg: &String) -> Self {
        Self {
            status: BuildStatus::ErrorBuilding,
            output: ArrayStringKV::default(),
            error_msg: string_to_cchar(msg),
        }
    }
}

impl From<ArrayStringKV> for BuildResult {
    fn from(value: ArrayStringKV) -> Self {
        Self {
            status: BuildStatus::Success,
            output: value,
            error_msg: Default::default(),
        }
    }
}

impl From<ConfigParam> for *const BuildResult {
    fn from(value: ConfigParam) -> Self {
        let r = BuildResult {
            status: BuildStatus::Success,
            output: value.into(),
            error_msg: Default::default(),
        };

        Box::leak(Box::new(r))
    }
}

impl Into<*const BuildResult> for BuildResult {
    fn into(self) -> *const BuildResult {
        Box::leak(Box::new(self))
    }
}
