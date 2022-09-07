pub mod engine_client_v1;
pub mod engine_client_v2;
use engine_client_v1::EngineClientV1;
use engine_client_v2::EngineClientV2;
use mxyz_engine::config::engine_runner_variant::EngineRunnerVariant;

pub struct Client {
    runner_variant: EngineRunnerVariant,
    engine_runner: Box<dyn EngineClient>, // TODO rather use enum?
}
impl Client {
    pub fn new(runner_variant: usize) -> Self {
        let runner_variant = match runner_variant {
            0 => EngineRunnerVariant::LocalRust,
            1 => EngineRunnerVariant::ClientWASM,
            2 => EngineRunnerVariant::ServerRust,
            _ => todo!(),
        };
        let engine_runner: Box<dyn EngineClient> = match runner_variant {
            EngineRunnerVariant::ClientWASM => {
                let engine_runner = EngineClientV1::new();
                Box::new(engine_runner)
            }
            EngineRunnerVariant::ServerRust => {
                let engine_runner = EngineClientV2::new();
                Box::new(engine_runner)
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

use async_trait::async_trait; // TODO needed?
#[async_trait] // TODO needed?
pub trait EngineClient {
    fn init(&mut self, category: &str, simulation_variant: &str);
}
