use std::collections::HashMap;

use serde::ser::Serialize;

/// A configuration parameter
#[derive(Debug, PartialEq, Clone)]
pub enum ConfigParam {
    Boolean(bool),
    HashMap(HashMap<String, ConfigParam>),
    Int(i64),
    Null,
    String(String),
    Vec(Vec<ConfigParam>),
}

impl Serialize for ConfigParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        // let mut s = serializer.serialize_struct("ConfigParam", 3)?;
        match self {
            ConfigParam::Boolean(v) => v.serialize(serializer),
            ConfigParam::HashMap(v) => v.serialize(serializer),
            ConfigParam::Int(v) => v.serialize(serializer),
            ConfigParam::Null => serializer.serialize_none(),
            ConfigParam::String(v) => v.serialize(serializer),
            ConfigParam::Vec(v) => v.serialize(serializer),
        }
    }
}

/// Returns a human-readable type
pub fn config_param_type_to_str(cfg: &ConfigParam) -> &str {
    match cfg {
        ConfigParam::Boolean(_) => "boolean",
        ConfigParam::HashMap(_) => "hashmap",
        ConfigParam::Int(_) => "integer",
        ConfigParam::Null => "null",
        ConfigParam::String(_) => "string",
        ConfigParam::Vec(_) => "vector",
    }
}

/// Returns a parameter by key.
/// Unlike config_param_consume_hashmap_key, doesn't remove the key from collection
pub fn config_param_get_hashmap_key<S: Into<String>>(cfg: &ConfigParam, key: S)
        -> Result<Option<ConfigParam>, String> {
    match cfg {
        ConfigParam::HashMap(m) => {
            match m.get(&key.into()) {
                Some(x) => Ok(Some(x.clone())),
                None => Ok(None),
            }
            
        },
        _ => return Err(format!("The provided config is not a hashmap: {:?}", cfg)),
    }
}

/// Returns a parameter by key and removes the key from colection
pub fn config_param_consume_hashmap_key<S: Into<String>>(cfg: &mut ConfigParam, key: S)
        -> Result<Option<ConfigParam>, String> {
    match cfg {
        ConfigParam::HashMap(m) => {
            match m.remove(&key.into()) {
                Some(x) => Ok(Some(x)),
                None => Ok(None),
            }
        },
        _ => return Err(format!("The provided config is not a hashmap: {:?}", cfg)),
    }
}

/// Merges two configuration params into new instance of configuration params
/// Collections are merged for sure. In case of scalar values - return the second value
pub fn config_params_merge(first: &ConfigParam, second: &ConfigParam) -> Result<ConfigParam, String> {
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
                        let merged = config_params_merge(first_nested, second_nested)?; // TODO: do NOT clone. Use borrowed vals instead?
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

///
pub fn config_param_into_string_kv_hashmap(config_param: ConfigParam) -> Result<Option<HashMap<String, String>>, String> {
    let config_param_map = match config_param {
        ConfigParam::HashMap(m) => m,
        _ => return Err(format!("Cannot convert the configuration into hashmap: the provided config is not of map type")),
    };
    let v: HashMap<String, String> = config_param_map.iter()
        .map(|(k, v)| {
            let v2 = match v {
                ConfigParam::String(v) => v.clone(),
                ConfigParam::Int(v) => format!("{}", v),
                _ => return Err(format!("Invalid data type of environment variable '{}'. It must be string or integer", k.clone()))
            };
            Ok((k.clone(), v2))
        })
        .collect::<Result<_, _>>()?;
    Ok(Some(v))
}

/// Returns a property of provided config as a haspmap where keys and values are strings.
/// Returns an error if provided collection or requested key are not hashmaps.
pub fn config_param_get_key_as_string_kv_hashmap<S: Into<String>>(config_param: &ConfigParam, key: S) -> Result<Option<HashMap<String, String>>, String> {
    let key: String = key.into();
    let config_param_map = match config_param {
        ConfigParam::HashMap(m) => m,
        _ => return Err(format!("Cannot read a key '{}' into hashmap: the type of provided config is not hashmap", key)),
    };

    let config_param_map = match config_param_map.get(&key) {
        Some(x) => x.clone(),
        None => return Ok(None),
    };
    let config_param_map = match config_param_into_string_kv_hashmap(config_param_map)? {
            Some(x) => Some(x.clone()),
            None => None,
    };
    Ok(config_param_map)
}

pub fn config_param_get_key_remove_key_from_hashmap<S: Into<String>>(config_param: &mut ConfigParam, key: S) -> Result<Option<ConfigParam>, String> {
    let k = key.into();
    match config_param {
        ConfigParam::HashMap(kv) => Ok(kv.remove(&k)),
        _ => Err(format!("Cannot delete a key '{}' from config: the provided config is not a hashmap", &k))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
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


        let merged = match config_params_merge(&first, &second) {
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