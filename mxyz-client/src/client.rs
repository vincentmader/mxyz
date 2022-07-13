use crate::renderer::Renderer;
use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
use mxyz_config::ClientConfig;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

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
    pub fn new(client_id: usize) -> Self {
        let channel_ws_to_rndr = mpsc::channel();
        let (tx_web_to_render, rx_web_to_render) = channel_ws_to_rndr;

        let config = ClientConfig::new(client_id);
        let renderer = Renderer::new();
        let websocket = WebSocketClient::new(HOST, PORT, tx_web_to_render);

        SimulationClientV1 {
            config,
            renderer,
            websocket,
        }
    }
    /// Initializes Renderer-Client
    pub fn init(&mut self, simulation_variant: &str) {
        dom::set_panic_hook();
        self.renderer.init();
        self.websocket.init().unwrap();
    }
    /// Runs Renderer-Client in Animation Loop
    pub async fn run(mut self) {
        let arc = Arc::new(Mutex::new(&mut self));
        // DB Sync via TCP (WebSocket)
        let _ = self.loop_state_getter().await;
        // let _ = self.loop_state_getter(&arc).await;
        // let _ = arc.clone().lock().unwrap().loop_state_getter().await;

        // Renderer
        let _ = self.loop_state_renderer().await;
        // let _ = self.loop_state_renderer(&arc).await;
        // let _ = arc.clone().lock().unwrap().loop_state_renderer().await;

        // - both need access to same state-vec: -> ArcMutex
    }
    async fn loop_state_getter(
        &self,
        // arc: &'static Arc<Mutex<&mut SimulationClientV1>>,
    ) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // let arc = arc.lock().unwrap();
            // if arc.config.frame_id.0 > arc.config.frame_id.1 {
            //     let _ = f.borrow_mut().take();
            //     return;
            // }
            dom::console_log!("\t\t{:?}", i);
            // std::thread::spawn(|| {});
            // self.step(&tx); //
            // self.step(); //
            i += 1;
            dom::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        dom::request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }

    async fn loop_state_renderer(
        &self,
        // arc: &Arc<Mutex<&mut SimulationClientV1>>,
    ) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut i = 0;
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // if self.config.frame_id.0 > self.config.frame_id.1 {
            //     let _ = f.borrow_mut().take();
            //     return;
            // }
            dom::console_log!("{:?}", i);
            // std::thread::spawn(|| {});
            // self.step(&tx); //
            // self.step(); //
            i += 1;
            dom::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        dom::request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }

    /// Forwards Renderer to Next Time-Step
    pub fn step(&mut self) {
        let frame_id = self.config.frame_id.0;
        // tmp::draw(i); // TODO create renderer with loop over systems & entities
        self.config.frame_id.0 += 1;
    }
}

// fn foo<F>(h: F) -> Result<(), JsValue>
// where
//     F: FnMut() -> (),
// {
//     let f = Rc::new(RefCell::new(None));
//     let g = f.clone();
//     *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
//         h();
//         // if self.config.frame_id.0 > self.config.frame_id.1 {
//         //     let _ = f.borrow_mut().take();
//         //     return;
//         // }
//         // // std::thread::spawn(|| {});
//         // // self.step(&tx); //
//         // self.step(); //
//         dom::request_animation_frame(f.borrow().as_ref().unwrap());
//     }) as Box<dyn FnMut()>));
//     dom::request_animation_frame(g.borrow().as_ref().unwrap());
//     Ok(())
// }
