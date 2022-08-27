use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
use wasm_bindgen::prelude::*;

const WEBSOCKET_HOST: &str = "127.0.0.1";
const WEBSOCKET_PORT: u16 = 1234;

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    websocket: WebSocketClient,
}

#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    pub fn new() -> Self {
        let websocket = WebSocketClient::new(WEBSOCKET_HOST, WEBSOCKET_PORT);
        SimulationClientV1 { websocket }
    }

    /// Initializes Renderer-Client
    pub fn init(&mut self, category: &str, simulation_variant: &str) {
        let page_id = (category, simulation_variant);
        dom::console_log!("page-id: {:?}", page_id);
        dom::set_panic_hook();
        self.websocket.init().unwrap();
    }
}
