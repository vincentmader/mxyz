pub struct ClientConfig {
    pub frame_id: (usize, usize),
}
impl ClientConfig {
    pub fn new() -> Self {
        let frame_id = (0, usize::MAX);
        ClientConfig { frame_id }
    }
}
