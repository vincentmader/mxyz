use async_trait::async_trait; // TODO needed?
#[async_trait] // TODO needed?
pub trait EngineRunner {
    fn init(&mut self, category: &str, simulation_variant: &str);
}
