use std::env;

use crate::types::config_param::ConfigParam;

/// Arguments for configuration builder's build method.
#[derive(Default, Debug)]
pub struct BuildArgs {
    /// Context is injected into each iteration of configuration rendering,
    /// but it is not returned as part of the final configuration.
    pub context: Option<ConfigParam>,
    /// Defaults for configuration parameters. Applied at the first stage of configuration building.
    pub defaults: Option<ConfigParam>,
    /// Overrides for configuration parameters. Applied at the last stage of configuration building.
    pub overrides: Option<ConfigParam>,
    /// A list of paths to configuration files
    pub paths: Vec<String>,
}

impl BuildArgs {
    pub fn new_default() -> Self {
        Self::default()
    }

    pub fn with_context(mut self, context: ConfigParam) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_defaults(mut self, defaults: ConfigParam) -> Self {
        self.defaults = Some(defaults);
        self
    }

    pub fn with_overrides(mut self, overrides: ConfigParam) -> Self {
        self.overrides = Some(overrides);
        self
    }


    pub fn with_paths_separated<S: Into<String>>(mut self, paths: S) -> Self {
        self.paths = env::split_paths(&paths.into())
            .map(|p| p.into_os_string().into_string().unwrap())
            .collect();
        self
    }

    pub fn with_paths<S: Into<String>>(mut self, paths: Vec<S>) -> Self {
        self.paths = paths.into_iter().map(|p| p.into()).collect();
        self
    }
}
