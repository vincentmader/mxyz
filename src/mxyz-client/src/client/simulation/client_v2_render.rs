use super::EngineRunner;
use crate::config::ClientConfig;
use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
use std::future::Future;

/// Simulation-Client v2
/// - Receive states via WebSocket.
/// - Render to Canvas (Animation Loop).
pub struct SimulationClientV2Render {
    config: ClientConfig,
    websocket: WebSocketClient,
}

impl SimulationClientV2Render {
    /// Create new Simulation-Renderer-Client.
    /// - Create Client Config.
    /// - Create WebSocket Client.
    pub fn new() -> Self {
        // Create config.
        let config = ClientConfig::new();
        // Create WebSocket Client using host & port from Client Config.
        let websocket = WebSocketClient::new(&config.websocket_host, config.websocket_port);

        SimulationClientV2Render { config, websocket }
    }
}
impl EngineRunner for SimulationClientV2Render {
    /// Initialize Renderer-Client.
    /// - ... page-id
    /// - ... panic hook
    /// - Initialize WebSocket Client.
    fn init(&mut self, category: &str, simulation_variant: &str) {
        let page_id = (category, simulation_variant);

        // ...
        dom::set_panic_hook();

        // Initialize WebSocket Client.
        self.websocket.init().unwrap();
    }
}
