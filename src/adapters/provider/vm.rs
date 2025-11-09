use std::error::Error;
use std::str::FromStr;
use crate::core::config::ports::ConfigProvider;
use crate::core::resources::ports::DeployProvider;

pub(crate) struct VM {
    name: String,
    os: String,
}

impl VM {
    pub(crate) fn new() -> Self {
        Self {
            name: "".to_string(),
            os: "".to_string(),
        }
    }
}

impl DeployProvider for VM {
    fn deploy(self) -> Result<String, Box<dyn Error>> {
        todo!()
    }

    fn check_exists(self) -> Result<bool, Box<dyn Error>> {
        todo!()
    }
}
