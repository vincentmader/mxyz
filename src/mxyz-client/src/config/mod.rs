pub struct ClientConfig {
    pub websocket_host: String,
    pub websocket_port: u16,
    // pub client_id: usize,
}
impl ClientConfig {
    pub fn new() -> Self {
        let websocket_host = "127.0.0.1".into();
        let websocket_port = 1234;
        ClientConfig {
            websocket_host,
            websocket_port,
        }
    }
}

pub struct RendererConfig {}
