use super::EngineRunner;
use crate::utils::dom;
use mxyz_client_engine::SimulationEngineV1;
use mxyz_engine::engine::Engine;
use std::future::Future;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Simulation-Client v2
/// - Compute States
/// - Render to Canvas (Animation Loop).
pub struct SimulationClientV1Compute {
    engine: SimulationEngineV1,
}

#[wasm_bindgen]
impl SimulationClientV1Compute {
    /// Create new Simulation-Compute-Client.
    pub fn new() -> Self {
        let engine_id = 0;
        let engine = SimulationEngineV1::new(engine_id);
        SimulationClientV1Compute { engine }
    }
}
impl EngineRunner for SimulationClientV1Compute {
    /// Initialize Compute-Client.
    /// - ... page-id
    /// - ... panic hook
    fn init(&mut self, category: &str, simulation_variant: &str) {
        let page_id = (category, simulation_variant);
        dom::console_log!("page-id: {:?}", page_id);
        // ...
        dom::set_panic_hook();

        let simulation_variant = Some(simulation_variant.into());
        self.engine.init(simulation_variant);
    }
}
