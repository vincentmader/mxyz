use crate::config::ClientConfig;
use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    config: ClientConfig,
    websocket: WebSocketClient,
}

#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    pub fn new() -> Self {
        let config = ClientConfig::new();
        let websocket = WebSocketClient::new(&config.websocket_host, config.websocket_port);
        SimulationClientV1 { config, websocket }
    }

    /// Initializes Renderer-Client
    pub fn init(&mut self, category: &str, simulation_variant: &str) {
        let page_id = (category, simulation_variant);
        dom::console_log!("page-id: {:?}", page_id);
        dom::set_panic_hook();
        self.websocket.init().unwrap();
    }
}
