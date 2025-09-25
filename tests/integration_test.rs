extern crate configtpl;

use std::collections::HashMap;

use configtpl::{config_builder::ConfigBuilder, types::config_param::ConfigParam};



#[test]
fn test_builder_simple() {
    let builder = ConfigBuilder::new();

    let mut cfg_expected: HashMap<String, ConfigParam> = HashMap::new();
    cfg_expected.insert(String::from("simple_string_val"), ConfigParam::String(String::from("Simple String Value")));

    assert_eq!(cfg_expected, builder.build_from_files(&String::from("tests/config1.yaml"), &None, &None).unwrap());
}