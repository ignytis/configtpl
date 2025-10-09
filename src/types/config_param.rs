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

impl ConfigParam {
    /// Returns a human-readable type
    pub fn type_to_str(&self) -> &str {
        match self {
            ConfigParam::Boolean(_) => "boolean",
            ConfigParam::HashMap(_) => "hashmap",
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
}

impl Serialize for ConfigParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
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
