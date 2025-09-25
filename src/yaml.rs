use std::collections::HashMap;

use yaml_rust::{Yaml, YamlLoader};

use crate::types::config_param::ConfigParam;


/// Converts a YAML string to vector of ConfigParam objects
pub fn yaml_string_to_configs(yml: &String) -> Result<Vec<ConfigParam>, String> {
    let yaml_doc = match YamlLoader::load_from_str(yml) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to parse YAML: {}", e)),
    };

    let ymls = yaml_doc.iter().map(|doc| yaml_to_config(doc)).collect();
    match ymls {
        Ok(v) => Ok(v),
        Err(e) => Err(format!("Failed to load a YAML document: {}", e)),
    }
}

/// Converts the YAML document into ConfigParam.
/// We are using ConfigParam instead of YAML document in the code
/// in order to encapsulate the YAML library internals.
fn yaml_to_config(yml: &Yaml) -> Result<ConfigParam, String> {
    let result = match yml {
        Yaml::Alias(_) => return Err(format!("Unsupported type: alias in YAML")),
        Yaml::Array(v) => {
            let mut result_vec: Vec<ConfigParam> = Vec::with_capacity(v.len());
            for i in v {
                result_vec.push(yaml_to_config(i)?);
            }
            ConfigParam::Vec(result_vec)
        },
        Yaml::BadValue => return Err(format!("Bad value in YAML")),
        Yaml::Boolean(v) => ConfigParam::Boolean(*v),
        Yaml::Hash(kv) => {
            let mut result_map: HashMap<String, ConfigParam> = HashMap::new();
            for (k, v) in kv.iter() {
                // TODO: add checks in case if key is a NOT string
                let k = k.clone().into_string().unwrap();
                let v = yaml_to_config(v)?;
                result_map.insert(k, v);
            }
            ConfigParam::HashMap(result_map)
        }
        Yaml::Integer(v) => ConfigParam::Int(*v),
        Yaml::Null => ConfigParam::Null,
        // TODO: should we keep string here, the same as Yaml does,
        // or float is needed?
        Yaml::Real(v) => ConfigParam::String(v.clone()),
        Yaml::String(v) => ConfigParam::String(v.clone()),
    };
    Ok(result)
}