use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};

const CONFIG_FILE: &str = "config.yml";

pub struct ApplicationConfig {
    pub server_url: String,
    pub db_url: String,
    pub token_key: String,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        let mut yml_data = String::new();
        File::open(CONFIG_FILE)
            .expect("config.yam file not found")
            .read_to_string(&mut yml_data).unwrap();
        let doc = YamlLoader::load_from_str(&yml_data).unwrap();
        let server_url = get_cfg(&doc, "server_url", "127.0.0.1:4000");
        let db_url = get_cfg(&doc, "db_url", "tn.db");
        let token_key = get_cfg(&doc, "token_key", "secret");
        Self { server_url, db_url, token_key }
    }
}

fn get_cfg<'a>(doc: &'a Vec<Yaml>, key: &str, default: &str) -> String {
    for x in doc {
        match x {
            Yaml::Hash(hash) => {
                let v = hash.get(&Yaml::String(key.to_string()));
                if v.is_some() {
                    return v.unwrap().as_str().unwrap_or(default).to_string();
                }
            }
            _ => {}
        }
    }
    panic!(format!("in {} key: '{}' not exist!", CONFIG_FILE, key));
}
