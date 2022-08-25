pub struct ClientConfig {
    pub client_id: usize,
    pub frame_id: (usize, usize),
    // pub rx: Receiver<Package>,
}
impl ClientConfig {
    pub fn new(
        client_id: usize,
        // rx: Receiver<Package>
    ) -> Self {
        let frame_id = (0, usize::MAX);
        ClientConfig {
            client_id,
            frame_id,
            // rx,
        }
    }
}

pub struct RendererConfig {}
