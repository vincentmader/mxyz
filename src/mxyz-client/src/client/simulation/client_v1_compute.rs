use crate::utils::dom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Simulation-Client v2
/// - Compute States
/// - Render to Canvas (Animation Loop).
pub struct SimulationClientV1Compute {
    // engine: Engine,
}

#[wasm_bindgen]
impl SimulationClientV1Compute {
    /// Create new Simulation-Compute-Client.
    pub fn new() -> Self {
        SimulationClientV1Compute {}
    }

    /// Initialize Compute-Client.
    /// - ... page-id
    /// - ... panic hook
    pub fn init(&mut self, category: &str, simulation_variant: &str) {
        let page_id = (category, simulation_variant);
        dom::console_log!("page-id: {:?}", page_id);
        // ...
        dom::set_panic_hook();
    }
}
