use std::collections::HashMap;

use minijinja::Environment;

use crate::{types::config_param::{config_param_type_to_str, config_params_merge, ConfigParam}, yaml::yaml_string_to_configs};

pub struct ConfigBuilder<'a> {
    jinja_env: Environment<'a>
}



impl<'a> ConfigBuilder<'a> {
    pub fn new() -> Self {
        let jinja_env = Environment::new();

        Self {
            jinja_env,
        }
    }

    /// Builds the configuration from list of provided files.
    /// # Arguments
    /// * `paths` - a colon-separated list of paths to config files. Later values overwrite earlier ones.
    /// * `overrides` - an optional dictionary of overrides.
    /// * `ctx` - context. Context is not merged into configuration keys, but participates in rendering of values
    pub fn build_from_files(&self, paths: &String, overrides: &Option<ConfigParam>, ctx: &Option<ConfigParam>) -> Result<HashMap<String, ConfigParam>, String> {
        let paths: Vec<String> = paths.split(":").map(|p| String::from(p)).collect();
        let ctx = match ctx {
            Some(c) => c,
            None => &ConfigParam::HashMap(HashMap::new()),
        };

        let mut result: ConfigParam = ConfigParam::HashMap(HashMap::new());

        for path in &paths {
            let contents = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => return Err(format!("Failed to read the configuration file file '{}': {}", &path, e)),
            };

            let yaml_contents = match self.jinja_env.render_str(contents.as_str(), ctx) {
                Ok(r) => r,
                Err(e) => return Err(format!("Failed to render the configuration file '{}': {}", &path, e)),
            };

            for config_param_iter in yaml_string_to_configs(&yaml_contents)? {
                result = config_params_merge(&result, &config_param_iter)?;
            }
        }

        // Apply overrides
        if let Some(o) = overrides {
            result = config_params_merge(&result, o)?;
        }

        match result {
            ConfigParam::HashMap(v) => Ok(v),
            _ => Err(format!("The configuration is resolved into incorrect type: {}", config_param_type_to_str(&result)))
        }
    }
}