pub mod client_v1_engine_runner;
pub mod client_v2_engine_renderer;
use async_trait::async_trait;

#[async_trait]
pub trait EngineRunner {
    fn init(&mut self, category: &str, simulation_variant: &str);
}
