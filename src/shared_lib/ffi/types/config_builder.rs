use crate::{
    shared_lib::ffi::{types::{collections::Array, config_param::ConfigParam, std_types::ConstCharPtr}, utils::strings::cchar_to_string},
    types::config_builder::BuildArgs as LibBuildArgs
};

#[repr(C)]
#[derive(Default)]
pub struct BuildArgs {
    /// Context is injected into each iteration of configuration rendering,
    /// but it is not returned as part of the final configuration.
    pub context: *mut ConfigParam,
    /// Defaults for configuration parameters. Applied at the first stage of configuration building.
    pub defaults: *mut ConfigParam,
    /// Overrides for configuration parameters. Applied at the last stage of configuration building.
    pub overrides: *mut ConfigParam,
    /// A list of paths to configuration files
    pub paths: Array<ConstCharPtr>,
}

impl Into<LibBuildArgs> for BuildArgs {
    fn into(self) -> LibBuildArgs {
        let mut result: LibBuildArgs = LibBuildArgs::new_default();

        if !self.context.is_null() {
            result = result.with_context(unsafe { *(self.context) }.into());
        }
        if !self.defaults.is_null() {
            result = result.with_defaults(unsafe { *(self.defaults) }.into());
        }
        if !self.overrides.is_null() {
            result = result.with_overrides(unsafe { *(self.overrides) }.into());
        }
        if self.paths.len > 0 {
            let paths: Vec<String> = unsafe {
                (0..self.paths.len)
                    .map(|i| cchar_to_string(*self.paths.data.offset(i as isize) as *const i8))
                    .collect()
            };
            result = result.with_paths(paths);
        }

        result
    }
}
