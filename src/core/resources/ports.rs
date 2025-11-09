use std::error::Error;
use std::str::FromStr;
use crate::core::config::ports::ConfigProvider;

pub(crate) trait DeployProvider {
    fn deploy(self) -> Result<String, Box<dyn Error>>;
    fn check_exists(self) -> Result<bool, Box<dyn Error>>;
}