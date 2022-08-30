pub mod client_v1_compute;
pub mod client_v2_render;
use async_trait::async_trait;

#[async_trait]
pub trait EngineRunner {
    fn init(&mut self, category: &str, simulation_variant: &str);
}
