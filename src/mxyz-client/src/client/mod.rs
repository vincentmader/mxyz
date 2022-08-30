pub mod engine_renderer_client_v2;
pub mod engine_runner_client_v1;
use engine_renderer_client_v2::EngineRendererClientV2;
use engine_runner_client_v1::EngineRunnerClientV1;
use mxyz_engine::config::engine_runner_variant::EngineRunnerVariant;

pub struct Client {
    runner_variant: EngineRunnerVariant,
    engine_runner: Box<dyn EngineRunner>, // TODO rather use enum?
}
impl Client {
    pub fn new(runner_variant: usize) -> Self {
        let runner_variant = match runner_variant {
            0 => EngineRunnerVariant::LocalRust,
            1 => EngineRunnerVariant::ClientWASM,
            2 => EngineRunnerVariant::ServerRust,
            _ => todo!(),
        };
        let engine_runner: Box<dyn EngineRunner>;
        match runner_variant {
            EngineRunnerVariant::ClientWASM => {
                let res = EngineRunnerClientV1::new();
                engine_runner = Box::new(res);
            }
            EngineRunnerVariant::ServerRust => {
                let res = EngineRendererClientV2::new();
                engine_runner = Box::new(res);
            }
            _ => todo!(),
        };
        Client {
            runner_variant,
            engine_runner,
        }
    }
    pub async fn init(&mut self, category: &str, simulation_variant: &str) {
        self.engine_runner.init(category, simulation_variant);
    }
}

use async_trait::async_trait;

#[async_trait]
pub trait EngineRunner {
    fn init(&mut self, category: &str, simulation_variant: &str);
}
