use std::collections::HashMap;

use minijinja::Environment;

use crate::types::{config_builder::BuildArgs, config_param::ConfigParam};

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
    /// * `paths` - a list of paths to config files. Later values overwrite earlier ones.\
    ///    A system path list separator should be used (i.e. `:` on Unix and `;` on Windows).
    /// * `overrides` - an optional dictionary of overrides.
    /// * `ctx` - context. Context is not merged into configuration keys, but participates in rendering of values
    pub fn build(&self, args: &BuildArgs) -> Result<ConfigParam, String> {
        let ctx = match &args.context {
            Some(c) => &c,
            None => &ConfigParam::HashMap(HashMap::new()),
        };

        let mut result: ConfigParam = ConfigParam::HashMap(HashMap::new());
        for path in &args.paths {
            let contents = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => return Err(format!("Failed to read the configuration file file '{}': {}", &path, e)),
            };

            // Apply all the previous iterations to context
            let ctx_iter = ConfigParam::merge(ctx, &result);

            // Render the YAML document (might produce multiple files) and merge into result
            let yaml_contents = match self.jinja_env.render_str(contents.as_str(), ctx_iter) {
                Ok(r) => r,
                Err(e) => return Err(format!("Failed to render the configuration file '{}': {}", &path, e)),
            };
            for config_param_iter in ConfigParam::new_from_yaml_str(yaml_contents)? {
                result = ConfigParam::merge(&result, &config_param_iter)?;
            }
        }

        // Apply overrides
        if let Some(o) = &args.overrides {
            result = ConfigParam::merge(&result, o)?;
        }

        Ok(result)
    }
}
