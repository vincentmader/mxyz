pub struct RendererConfig {
    pub frame_id: (usize, usize),
}
impl RendererConfig {
    pub fn new() -> Self {
        let frame_id = (0, usize::MAX);
        RendererConfig { frame_id }
    }
}
