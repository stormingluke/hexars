use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::adapters::config::Config;
use crate::core::config::ports::ConfigProvider;
use crate::core::resources::ports::DeployProvider;

#[derive(Debug)]
pub(crate) struct Docker {
    name: String,
    image: String,
}

impl Docker {
    pub(crate) fn new() -> Self {
        Self {
            name: "".to_string(),
            image: "".to_string(),
        }
    }
}

impl DeployProvider for Docker {
    fn deploy(self) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    fn check_exists(self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }
}
