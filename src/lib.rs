use crate::adapters::factory;

mod adapters;
mod core;

pub fn run(s: &str) {
    println!("{}", s);
}

pub fn run_cli() {
    let factory = factory::Factory::new();
}