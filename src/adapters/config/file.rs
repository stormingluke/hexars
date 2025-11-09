use crate::core::config::ports::ConfigProvider;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::BufRead;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    kind: String,
    version: String,
    #[serde()]
    docker_config: DockerConfig,
    vm_config: VMConfig,
}

impl Config {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            kind: "".to_string(),
            version: "".to_string(),
            docker_config: DockerConfig {
                name: "".to_string(),
                image: "".to_string(),
                version: (0, 0, 0),
            },
            vm_config: VMConfig {
                name: "".to_string(),
                os: "".to_string(),
                version: (0, 0, 0),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerConfig {
    name: String,
    image: String,
    version: (i32, i32, i32),
}

#[derive(Debug, Serialize, Deserialize)]
struct VMConfig {
    name: String,
    os: String,
    version: (i32, i32, i32),
}

impl ConfigProvider for Config {
    fn write(self, name: String) -> Result<String, Box<dyn Error>> {
        println!("{:?}", self.name);
        Ok(String::from("ok"))
    }

    fn check_exists(self, name: &str) -> Result<bool, Box<dyn Error>> {
        let f = fs::OpenOptions::new().read(true).open(name).and_then(|f| {
            let reader = std::io::BufReader::new(f);
            let s = toml::from_slice::<Config>(reader.lines().next().unwrap().unwrap().as_bytes());

            Ok(())
        });

        Ok(true)
    }
}
