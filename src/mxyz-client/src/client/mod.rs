pub mod renderer_client_v2;
pub mod runner_client_v1;
use crate::engine::EngineRunner;
use mxyz_engine::config::engine_runner_variant::EngineRunnerVariant;
use renderer_client_v2::EngineRendererClientV2;
use runner_client_v1::EngineRunnerClientV1;

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
        let engine_runner: Box<dyn EngineRunner> = match runner_variant {
            EngineRunnerVariant::ClientWASM => {
                let engine_runner = EngineRunnerClientV1::new();
                Box::new(engine_runner)
            }
            EngineRunnerVariant::ServerRust => {
                let engine_runner = EngineRendererClientV2::new();
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
