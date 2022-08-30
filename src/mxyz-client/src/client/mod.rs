pub mod simulation;
use mxyz_engine::config::EngineRunnerVariant;
use simulation::client_v1_engine_runner::ClientV1EngineRunner;
use simulation::client_v2_engine_renderer::ClientV2EngineRenderer;
use simulation::EngineRunner;

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
                let res = ClientV1EngineRunner::new();
                engine_runner = Box::new(res);
            }
            EngineRunnerVariant::ServerRust => {
                let res = ClientV2EngineRenderer::new();
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
