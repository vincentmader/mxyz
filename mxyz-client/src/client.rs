use super::components::canvas::Canvas;
use super::config::ClientConfig;
use super::renderer::Renderer;
use super::utils::dom;
use crate::websocket_client;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::TcpSocket;

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    config: ClientConfig,
    renderer: Renderer,
    // http_client: reqwest::Client,
    // TODO server -> bytestream -> client
}
#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    pub fn new(client_id: usize) -> Self {
        let config = ClientConfig::new(client_id);
        let renderer = Renderer::new();
        // let http_client = reqwest::Client::new();
        SimulationClientV1 {
            config,
            renderer,
            // http_client,
        }
    }
    /// Initializes Renderer-Client
    pub fn init(&mut self, sim_id: &str) {
        dom::set_panic_hook();
        self.renderer.init();
    }
    /// Runs Renderer-Client in Animation Loop
    pub async fn run(mut self) -> Result<(), JsValue> {
        // TCP Client
        // TODO test get-request to server
        // - TCP get-requests (bytestream? -> decode)
        // - move inside animation loop (async?)
        websocket_client::start_websocket().unwrap();

        // ANIMATION LOOP
        // TODO move to utils/dom/mod.rs (?)
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if self.config.frame_id.0 > self.config.frame_id.1 {
                let _ = f.borrow_mut().take();
                return;
            }
            self.step(); //
            dom::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        dom::request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
    /// Forwards Renderer to Next Time-Step
    pub fn step(&mut self) {
        let i = self.config.frame_id.0;
        draw(i);
        self.config.frame_id.0 += 1;
    }
}

// TODO create renderer with loop over systems & entities
pub fn draw(i: usize) {
    let r = i as f64 * 0.3;
    let phi = i as f64 * 0.1;
    let pos = (r * phi.cos(), r * phi.sin());
    let pos = (pos.0 + 500., pos.1 + 500.);

    let mut canvas = Canvas::new(0);
    canvas.set_stroke_style("blue");
    canvas.set_fill_style("blue");
    canvas.draw_circle(pos, 4., true);
}
