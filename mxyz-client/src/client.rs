use super::renderer::Renderer;
use super::utils::dom;
use wasm_bindgen::prelude::*;
// use super::utils::dom::console;

// ----------------------------------------------------------------------------

#[wasm_bindgen]
/// Simulation Client: Renderer
pub struct SimulationRendererClient {
    renderer: Renderer,
    // TODO server -> bytestream -> client
}
#[wasm_bindgen]
impl SimulationRendererClient {
    /// Create new Simulation Renderer Client
    pub fn new() -> Self {
        let renderer = Renderer::new();
        SimulationRendererClient { renderer }
    }
    /// Initialize Renderer Client
    pub fn init(&mut self) {
        dom::set_panic_hook();
        self.renderer.init();
    }
    /// Run Renderer Client
    pub fn run(&mut self) {}
}
