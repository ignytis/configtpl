use std::{collections::HashMap, env};

use serde::ser::Serialize;
use yaml_rust::{Yaml, YamlLoader};

/// A configuration parameter
#[derive(Debug, PartialEq, Clone)]
pub enum ConfigParam {
    Boolean(bool),
    HashMap(HashMap<String, ConfigParam>),
    Int(i64),
    Float(f64),
    Null,
    String(String),
    Vec(Vec<ConfigParam>),
}

impl ConfigParam {
    /// Converts a YAML string to vector of ConfigParam objects
    pub fn new_from_yaml_str<S: Into<String>>(s: S) -> Result<Vec<ConfigParam>, String> {
        let yaml_doc = match YamlLoader::load_from_str(&s.into()) {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed to parse YAML: {}", e)),
        };

        let ymls = yaml_doc.iter().map(|doc| yaml_to_config(doc)).collect();
        match ymls {
            Ok(v) => Ok(v),
            Err(e) => Err(format!("Failed to load a YAML document: {}", e)),
        }
    }

    /// Returns a new instance of ConfigParam, assuming that argument is a scalar value
    fn new_from_scalar_str_assuming_type(val: &str) -> ConfigParam {
        if val.starts_with('"') && val.ends_with('"') {
            ConfigParam::String(val[1..val.len() - 1].to_string())
        } else if val.eq_ignore_ascii_case("null") {
            ConfigParam::Null
        } else if val.eq_ignore_ascii_case("true") {
            ConfigParam::Boolean(true)
        } else if val.eq_ignore_ascii_case("false") {
            ConfigParam::Boolean(false)
        } else if let Ok(i) = val.parse::<i64>() {
            ConfigParam::Int(i)
        } else if let Ok(f) = val.parse::<f64>() {
            ConfigParam::Float(f)
        } else {
            ConfigParam::String(val.to_string())
        }
    }

    /// Builds a ConfigParam::HashMap from env vars with the given prefix (e.g., "MY_APP")
    pub fn new_from_env<S: Into<String>>(prefix: S) -> ConfigParam {
        let mut root = HashMap::new();
        let prefix_with_sep = format!("{}__", prefix.into());

        for (key, value) in env::vars() {
            if let Some(stripped) = key.strip_prefix(&prefix_with_sep) {
                let stripped = stripped.to_lowercase();
                let parts: Vec<&str> = stripped.split("__").collect();
                insert_nested(&mut root, &parts, &value);
            }
        }

        ConfigParam::HashMap(root)
    }

    /// Returns a human-readable type
    pub fn type_to_str(&self) -> &str {
        match self {
            ConfigParam::Boolean(_) => "boolean",
            ConfigParam::HashMap(_) => "hashmap",
            ConfigParam::Float(_) => "float",
            ConfigParam::Int(_) => "integer",
            ConfigParam::Null => "null",
            ConfigParam::String(_) => "string",
            ConfigParam::Vec(_) => "vector",
        }
    }

    /// Merges two configuration params into new instance of configuration params
    /// Collections are merged for sure. In case of scalar values - return the second value
    pub fn merge(first: &ConfigParam, second: &ConfigParam) -> Result<ConfigParam, String> {
        match first {
            ConfigParam::HashMap(m_first) => {
                match second {
                    ConfigParam::HashMap(m_second) => {
                        let mut result: HashMap<String, ConfigParam> = HashMap::new();
                        let keys_intersect: Vec<&String> = m_first.keys().filter(|k| m_second.keys().any(|k2| &k2 == k)).collect();
                        for (k, v) in m_first {
                            if keys_intersect.contains(&k) {
                                continue
                            }
                            result.insert(k.clone(), v.clone());
                        }
                        for (k, v) in m_second {
                            if keys_intersect.contains(&k) {
                                continue
                            }
                            result.insert(k.clone(), v.clone());
                        }
                        for k in keys_intersect {
                            let first_nested = m_first.get(k).unwrap();
                            let second_nested = m_second.get(k).unwrap();
                            let merged = ConfigParam::merge(first_nested, second_nested)?; // TODO: do NOT clone. Use borrowed vals instead?
                            result.insert(k.clone(), merged);
                        }
                        Ok(ConfigParam::HashMap(result))
                    },
                    _ => return Err(String::from("The first item is hashmap, the second is not")),
                }
            },
            ConfigParam::Vec(v_first) => {
                match second {
                    ConfigParam::Vec(v_second) => {
                        Ok(ConfigParam::Vec(v_first.iter().chain(v_second.iter()).cloned().collect()))
                    },
                    _ => return Err(String::from("The first item is vector, the second is not")),
                }
            },
            _ => Ok(second.clone()),
        }
    }

    /// Debug printing the config param
    pub fn debug_print(&self, prefix: Option<String>) {
        let prefix = prefix.unwrap_or_default();
        match self {
            ConfigParam::Boolean(v) => println!("{}: {}", prefix, v),
            ConfigParam::HashMap(v) => {
                for (k, v) in v {
                    v.debug_print(Some(format!("{}.{}", prefix, k)));
                }
            },
            ConfigParam::Float(v) => println!("{}: {}", prefix, v),
            ConfigParam::Int(v) => println!("{}: {}", prefix, v),
            ConfigParam::Null => println!("{}: null", prefix),
            ConfigParam::String(v) => println!("{}: {}", prefix, v),
            ConfigParam::Vec(v) => {
                for (i, item) in v.iter().enumerate() {
                    item.debug_print(Some(format!("{}[{}]", prefix, i)));
                }
            },
        }
    }
}

impl Serialize for ConfigParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        match self {
            ConfigParam::Boolean(v) => v.serialize(serializer),
            ConfigParam::HashMap(v) => v.serialize(serializer),
            ConfigParam::Float(v) => v.serialize(serializer),
            ConfigParam::Int(v) => v.serialize(serializer),
            ConfigParam::Null => serializer.serialize_none(),
            ConfigParam::String(v) => v.serialize(serializer),
            ConfigParam::Vec(v) => v.serialize(serializer),
        }
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



/// Insert a value into the nested hashmap structure
fn insert_nested(
    map: &mut HashMap<String, ConfigParam>,
    keys: &[&str],
    value: &str,
) {
    if keys.is_empty() {
        return;
    }
    if keys.len() == 1 {
        map.insert(keys[0].to_string(), ConfigParam::new_from_scalar_str_assuming_type(value));
    } else {
        let entry = map.entry(keys[0].to_string()).or_insert_with(|| {
            ConfigParam::HashMap(HashMap::new())
        });
        if let ConfigParam::HashMap(submap) = entry {
            insert_nested(submap, &keys[1..], value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_params_merge() {
        let mut first: HashMap<String, ConfigParam> = HashMap::new();
        let mut first_two: HashMap<String, ConfigParam> = HashMap::new();
        first_two.insert(format!("first_two_one"), ConfigParam::Null);
        first.insert(format!("first_one"), ConfigParam::Int(123));
        first.insert(format!("shared_two"), ConfigParam::HashMap(first_two));
        let first = ConfigParam::HashMap(first);

        let mut second: HashMap<String, ConfigParam> = HashMap::new();
        let mut second_two: HashMap<String, ConfigParam> = HashMap::new();
        second_two.insert(format!("second_two_one"), ConfigParam::Boolean(true));
        second.insert(format!("second_one"), ConfigParam::String(format!("Hello")));
        second.insert(format!("shared_two"), ConfigParam::HashMap(second_two));
        let second = ConfigParam::HashMap(second);


        let merged = match ConfigParam::merge(&first, &second) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };

        let merged = match merged {
            ConfigParam::HashMap(kv) => kv,
            _ => panic!("Unexpected type of merged config"),
        };
        match merged.get("first_one") {
            Some(v) => match v {
                ConfigParam::Int(v2) => {
                    assert_eq!(*v2, 123);
                },
                _ => panic!("Unexpected type of first_one. Must be integer")
            },
            None => panic!("first_one not found")
        }

        let shared_two = match merged.get("shared_two") {
            Some(ConfigParam::HashMap(v)) => v,
            _ => panic!("shared_two not found or is not a hashmap"),
        };
        assert_eq!(Some(&ConfigParam::Null), shared_two.get("first_two_one"));
        assert_eq!(Some(&ConfigParam::Boolean(true)), shared_two.get("second_two_one"));
    }
}
