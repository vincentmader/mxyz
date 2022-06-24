pub struct ClientConfig {
    pub client_id: usize,
    pub frame_id: (usize, usize),
}
impl ClientConfig {
    pub fn new(client_id: usize) -> Self {
        let frame_id = (0, usize::MAX);
        ClientConfig {
            client_id,
            frame_id,
        }
    }
}
