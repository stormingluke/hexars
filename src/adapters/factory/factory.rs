use crate::adapters::config;
use crate::adapters::config::*;
use crate::adapters::provider::{docker::Docker, vm::VM};
use crate::core::{
    config::{models as cfgmodels, ports as cfgports, service as cfgservice},
    resources::{models as resmodels, ports as resports, service as resservice},
};

pub struct Factory {}

impl Factory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Factory {
    pub fn config_factory() -> impl cfgports::ConfigProvider {
        config::Config::new()
    }
    pub fn deploy_factory(
        kind: &str,
    ) -> Result<Box<dyn resports::DeployProvider>, Box<dyn std::error::Error>> {
        match kind {
            "docker" => Ok(Box::new(Docker::new())),
            "vm" => Ok(Box::new(VM::new())),
            _ => Err("Invalid provider".into()),
        }
    }
}
