pub mod simulation;
use mxyz_engine::config::EngineRunnerVariant;
use simulation::client_v1_compute::SimulationClientV1Compute;
use simulation::client_v2_render::SimulationClientV2Render;
use simulation::EngineRunner;
use wasm_bindgen::prelude::*;

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
                let res = SimulationClientV2Render::new();
                engine_runner = Box::new(res);
            }
            EngineRunnerVariant::ServerRust => {
                let res = SimulationClientV1Compute::new();
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

#[wasm_bindgen]
pub async fn init_client(runner_variant: usize, category: String, simulation_variant: String) {
    let mut client = Client::new(runner_variant);
    client.init(&category, &simulation_variant).await;
}
