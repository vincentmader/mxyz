use super::config::ClientConfig;
use super::renderer::Renderer;
use super::utils::dom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    config: ClientConfig,
    renderer: Renderer,
}
#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    pub fn new(client_id: usize) -> Self {
        let config = ClientConfig::new(client_id);
        let renderer = Renderer::new();
        SimulationClientV1 { config, renderer }
    }
    /// Initializes Renderer-Client
    pub fn init(&mut self, simulation_variant: &str) {
        dom::set_panic_hook();
        self.renderer.init();
    }
    /// Runs Renderer-Client in Animation Loop
    pub async fn run(mut self) -> Result<(), JsValue> {
        // let (tx, rx) = mpsc::channel::<Package>();

        // TCP Client
        // TODO test get-request to server
        // - TCP get-requests (bytestream? -> decode)
        // - move inside animation loop (async?)
        crate::websocket::start_client().unwrap();
        // crate::websocket::start_client(rx).unwrap();

        // ANIMATION LOOP
        // TODO move to utils/dom/mod.rs (?)
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if self.config.frame_id.0 > self.config.frame_id.1 {
                let _ = f.borrow_mut().take();
                return;
            }
            // std::thread::spawn(|| {});
            // self.step(&tx); //
            self.step(); //
            dom::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        dom::request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
    /// Forwards Renderer to Next Time-Step
    // pub fn step(&mut self, tx: &mpsc::Sender<Package>) {
    pub fn step(&mut self) {
        let frame_id = self.config.frame_id.0;
        // tmp::draw(i); // TODO create renderer with loop over systems & entities
        if frame_id % 100 == 0 {
            // let package = Package::Command(Command::SaveStatesToDatabase);
            // tx.send(package).unwrap();
        }
        self.config.frame_id.0 += 1;
    }
}
