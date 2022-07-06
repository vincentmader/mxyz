use crate::config::ClientConfig;
use crate::renderer::Renderer;
use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
use mxyz_universe::preset::SimulationVariant;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use std::sync::mpsc;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 1234;

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    config: ClientConfig,
    renderer: Renderer,
    websocket: WebSocketClient,
    // TODO add mpsc channel between websocket & config?
}
#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    pub fn new(simulation_variant: &str) -> Self {
        // let channel_ws_to_rndr = mpsc::channel();
        // let (tx_web_to_render, rx_web_to_render) = channel_ws_to_rndr;

        let simulation_variant = SimulationVariant::from(simulation_variant);

        let renderer = Renderer::new(
            &simulation_variant,
            // rx_web_to_render
        );
        let config = ClientConfig::new(simulation_variant);
        let websocket = WebSocketClient::new(
            HOST, PORT,
            // tx_web_to_render
        );

        // let websocket = WebSocketClient::new(HOST, PORT, tx_web_to_render, &mut renderer);

        SimulationClientV1 {
            config,
            renderer,
            websocket,
        }
    }
    /// Initializes Renderer-Client
    pub fn init(&mut self) {
        dom::set_panic_hook();
        self.renderer.init();
        self.websocket.init().unwrap();
    }
    /// Runs Renderer-Client in Animation Loop
    pub async fn run(mut self) -> Result<(), JsValue> {
        // let (tx, rx) = mpsc::channel::<Package>();

        // TCP Client
        // TODO test get-request to server
        // - TCP get-requests (bytestream? -> decode)
        // - move inside animation loop (async?)
        // crate::websocket::start_client().unwrap();
        // let mut client = crate::websocket::client::WebSocketClient::new(HOST, PORT);

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
    pub fn step(&mut self) {
        let _frame_id = self.config.frame_id.0;
        // tmp::draw(i); // TODO create renderer with loop over systems & entities
        self.config.frame_id.0 += 1;
    }
}
