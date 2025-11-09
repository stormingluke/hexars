use crate::adapters::factory::Factory;
use crate::core::config::ports::ConfigProvider;

struct Service {
    factory: Factory,
    config_provider: Box<dyn ConfigProvider>,
}

impl Service {
    pub fn new(factory: Factory, config_provider: Box<dyn ConfigProvider>) -> Self {
        Self {
            factory,
            config_provider,
        }
    }
}