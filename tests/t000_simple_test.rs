extern crate configtpl;

use std::collections::HashMap;

use configtpl::{config_builder::ConfigBuilder, types::{config_builder::BuildArgs, config_param::ConfigParam}};



#[test]
fn test_builder_simple() {
    let builder = ConfigBuilder::new();

    let mut cfg_urls: HashMap<String, ConfigParam> = HashMap::new();
    cfg_urls.insert(String::from("base"), ConfigParam::String(String::from("example.com")));
    cfg_urls.insert(String::from("mail"), ConfigParam::String(String::from("mail.example.com")));

    let mut cfg_server: HashMap<String, ConfigParam> = HashMap::new();
    cfg_server.insert(String::from("host"), ConfigParam::String(String::from("example.com")));
    cfg_server.insert(String::from("port"), ConfigParam::Int(1234));


    let mut cfg: HashMap<String, ConfigParam> = HashMap::new();
    cfg.insert(String::from("urls"), ConfigParam::HashMap(cfg_urls));
    cfg.insert(String::from("server"), ConfigParam::HashMap(cfg_server));

    assert_eq!(ConfigParam::HashMap(cfg),
               builder.build(&BuildArgs::default().with_paths_separated("tests/t000_simple/config.cfg")).unwrap());
}
