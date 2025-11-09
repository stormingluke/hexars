use std::error::Error;
use std::io::BufRead;
use std::str::FromStr;

pub(crate) trait ConfigProvider {
    fn write(self, name: String) -> Result<String, Box<dyn Error>>;
    fn check_exists(self, name: &str) -> Result<bool, Box<dyn Error>>;
}