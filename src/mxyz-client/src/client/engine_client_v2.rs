use super::EngineClient;
use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
// use std::future::Future;

const WS_HOST: &str = "127.0.0.1";
const WS_PORT: u16 = 1234;

/// Simulation-Client v2
///
/// Function:
/// - Receive states via WebSocket.
/// - Render to Canvas (in Animation Loop).
pub struct EngineClientV2 {
    websocket: WebSocketClient,
}

impl EngineClientV2 {
    /// Create new Simulation-Renderer-Client.
    /// - Create Client Config.
    /// - Create WebSocket Client.
    pub fn new() -> Self {
        // Create WebSocket Client using host & port from Client Config.
        let websocket = WebSocketClient::new(WS_HOST, WS_PORT);

        EngineClientV2 { websocket }
    }
}
impl EngineClient for EngineClientV2 {
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
