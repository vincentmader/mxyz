use crate::config::ClientConfig;
use crate::renderer::Renderer;
use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
use mxyz_engine_universe::preset::SimulationVariant;
use mxyz_engine_universe::state::SizedState;
use mxyz_engine_universe::state::StateQuery;
use mxyz_network::tcp_pkg::request;
use mxyz_network::tcp_pkg::response::Response;
use mxyz_network::tcp_pkg::TcpPackage;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebSocket;
use web_sys::{ErrorEvent, MessageEvent};

const HOST: &str = "127.0.0.1";
const PORT: u16 = 1234;
// const STATE_BATCH_SIZE: i32 = 50;

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    // config: ClientConfig,
    // renderer: Renderer,
    websocket: WebSocketClient,
}
#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    // pub fn new(client_id: usize) -> Self {
    pub fn new() -> Self {
        // let config = ClientConfig::new(client_id);
        // let renderer = Renderer::new();
        let websocket = WebSocketClient::new(HOST, PORT);
        SimulationClientV1 {
            // config,
            // renderer,
            websocket,
        }
    }
    /// Initializes Renderer-Client
    pub fn init(&mut self, _simulation_variant: &str) {
        dom::set_panic_hook();
        // self.renderer.init();
        self.websocket.init().unwrap();
    }

    // /// Runs Renderer-Client in Animation Loop
    // pub async fn run(self) {
    //     let states: Vec<SizedState> = vec![];
    //     let states = Arc::new(Mutex::new(states));

    //     // self.get_states(states.clone()).await;
    //     // self.render_states(states).await;
    // }

    // async fn render_states(self, states: Arc<Mutex<Vec<SizedState>>>) {
    //     let f = Rc::new(RefCell::new(None));
    //     let g = f.clone();
    //     *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //         let states = states.clone();
    //         let states = states.lock().unwrap();
    //         match states.len() {
    //             0 => {}
    //             _ => {
    //                 let idx = states.len() - 1;
    //                 let state = states.get(idx).unwrap();
    //                 // Self::render_state(&state);
    //             }
    //         }

    //         // let state_query = StateQuery::Between(0, 100);
    //         // Start sync-loop for this engine's states.
    //         // let request = request::Request::GetUpdatedStates(engine_id, state_query);
    //         // let request = TcpPackage::Request(request).to_bytes();
    //         // let states = cloned_ws.send_with_u8_array(&request).unwrap();
    //         // arc.lock()
    //         //     .unwrap()
    //         //     .send_with_u8_array(&request)
    //         //     .expect("send failed");
    //         dom::request_animation_frame(f.borrow().as_ref().unwrap());
    //     }) as Box<dyn FnMut()>));
    //     dom::request_animation_frame(g.borrow().as_ref().unwrap());
    // }

    // fn render_state(state: &SizedState) {
    //     dom::console_log!("state-id {:?}\n{:?}", state.state_id, state);
    // }
    // async fn get_states(&self, states: Arc<Mutex<Vec<SizedState>>>) {
    //     let mut states = states.lock().unwrap();
    // }

    // loop {
    // TODO Get states.
    // let state_id = 0;
    // let engine_id = 1; // TODO
    //                    // let STATE_BATCH_SIZE = 10;
    //                    // Request next states.
    //                    // let ws = self.websocket.socket.clone();
    //                    // dom::console_log!("Requesting States...");
    //                    // let state_query = StateQuery::Between(state_id as i32, state_id as i32 + STATE_BATCH_SIZE);
    //                    // let request = request::Request::GetUpdatedStates(engine_id, state_query);
    //                    // dom::console_log!("{:?}", request);
    //                    // let request = TcpPackage::Request(request).to_bytes();
    //                    // ws.send_with_u8_array(&request).unwrap();

    // let mut last_sync = 0;

    // let cloned_ws = self.websocket.socket.clone();
    // let arc = Arc::new(Mutex::new(cloned_ws));

    // // TODO Display states.
    // // }
    // let f = Rc::new(RefCell::new(None));
    // let g = f.clone();
    // let mut i = 0;
    // *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //     // let arc = arc.lock().unwrap();
    //     // if arc.config.frame_id.0 > arc.config.frame_id.1 {
    //     //     let _ = f.borrow_mut().take();
    //     //     return;
    //     // }
    //     dom::console_log!("\t\t{:?} (state-getter-step)", i);

    //     let state_query = StateQuery::Between(0, 100);
    //     // Start sync-loop for this engine's states.
    //     let request = request::Request::GetUpdatedStates(engine_id, state_query);
    //     let request = TcpPackage::Request(request).to_bytes();
    //     // let states = cloned_ws.send_with_u8_array(&request).unwrap();
    //     arc.lock()
    //         .unwrap()
    //         .send_with_u8_array(&request)
    //         .expect("send failed");

    //     // for state in states.iter() {}
    //     last_sync = 1;

    //     // TODO get states from here?
    //     // std::thread::spawn(|| {});
    //     // self.step(&tx); //
    //     // self.step(); //
    //     i += 1;
    //     dom::request_animation_frame(f.borrow().as_ref().unwrap());
    // }) as Box<dyn FnMut()>));
    // dom::request_animation_frame(g.borrow().as_ref().unwrap());

    // =============================================================================

    // // let _client_id = 0; // TODO
    // let address = format!("ws://{}:{}", HOST, PORT);
    // let ws = WebSocket::new(&address).unwrap();

    // let cloned_ws = ws.clone();
    // let onopen_callback = Closure::wrap(Box::new(move |_| {
    //     dom::console_log!("TCP socket opened.");
    //     // Add new client.
    //     let request = TcpPackage::Request(request::Request::AddClient).to_bytes();
    //     dom::console_log!("Requesting new Client...");
    //     cloned_ws.send_with_u8_array(&request).unwrap();
    // }) as Box<dyn FnMut(JsValue)>);
    // ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    // onopen_callback.forget();

    // let cloned_ws = ws.clone();
    // let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
    //     dom::console_log!("ERROR Event: {:?}", e);
    // }) as Box<dyn FnMut(ErrorEvent)>);
    // cloned_ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    // onerror_callback.forget();

    // // let mut states_a: Vec<SizedState> = vec![];
    // // let states_a = Arc::new(Mutex::new(states_a));
    // // let states_b = states_a.clone();

    // let mut cloned_ws = ws.clone();
    // let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
    //     // Handle ArrayBuffer.
    //     if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
    //         let array = js_sys::Uint8Array::new(&abuf);
    //         let _len = array.byte_length() as usize;
    //         let package = TcpPackage::from_bytes(array.to_vec());

    //         // // console_log!("\nArraybuffer received {} bytes", len);
    //         // // console_log!("\nArraybuffer received {} bytes -> {:?}", len, &package);
    //         match &package {
    //             TcpPackage::Response(res) => match &res {
    //                 Response::StateVector(engine_id, states) => {
    //                     for s in states.iter() {
    //                         // states_b.lock().unwrap().push(s.clone());
    //                     }
    //                     // states_a = *states;
    //                 }
    //                 _ => todo!(),
    //             },
    //             _ => todo!(),
    //         }

    //         handle_onmessage_package(&mut cloned_ws, package);

    //         // Handle Blob.
    //     } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
    //         dom::console_log!("message event, received Blob: {:?}", blob);
    //     // Handle Text.
    //     } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
    //         dom::console_log!("message event, received Text: {:?}", txt);
    //     // Handle Other.
    //     } else {
    //         dom::console_log!("message event, received Unknown: {:?}", e.data());
    //     }
    // }) as Box<dyn FnMut(MessageEvent)>);
    // ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // onmessage_callback.forget();

    // =============================================================================

    // // use crate::renderer::components::canvas::Canvas;
    // // let mut canvas = Canvas::new(0);
    // // let cnv_dim = canvas.dimensions;
    // // canvas.init();
    // // canvas.set_fill_style("purple");
    // // canvas.set_stroke_style("purple");

    // // let canvas = Arc::new(Mutex::new(canvas));
    // // canvas
    // //     .clone()
    // //     .lock()
    // //     .unwrap()
    // //     .draw_circle((100., 100.), 3., true);

    // // dom::console_log!("{:?}", states_a.clone().lock().unwrap().len());
    // // for state in states_a.clone().lock().unwrap().iter() {
    // //     dom::console_log!("{:?}", state);
    // //     //       // canvas.clear();
    // //     //      // let text = format!("state {}", state.state_id);
    // //     //      // let (x, y) = (50., 50.);
    // //     //      // canvas.fill_text(&text, x, y);

    // //     for system in state.systems.iter() {
    // //         match &system.variant {
    // //             SizedSystemVariant::EntitiesV1(system) => {
    // //                 // for planet in system.entities.iter() {
    // //                 //     dom::console_log!("planet {:?}", planet);
    // //                 //     let pos = planet.position;
    // //                 //     let pos = (pos[0], pos[1]);
    // //                 //     let pos = (pos.0 * cnv_dim.0 / 2., pos.1 * cnv_dim.1 / 2.);
    // //                 //     let pos = (pos.0 + cnv_dim.0 / 2., pos.1 + cnv_dim.1 / 2.);
    // //                 //     let r = 1.;
    // //                 //     // canvas.clone().lock().unwrap().draw_circle(pos, r, true);
    // //                 // }
    // //             }
    // //             _ => todo!(),
    // //         }
    // //     }
    // // }
    // }

    // pub fn handle_onmessage_package(ws: &mut WebSocket, package: TcpPackage) -> usize {
    // match package {
    // TcpPackage::Response(res) => {
    //     match res {
    //         Response::AddedClient(client_id) => {
    //             dom::console_log!("New Client confirmed. (id={:?})", client_id);
    //             // TODO load client-id into client struct field
    //             // self.client_id = Some(client_id);
    //             // TODO Get simulation-variant from HTML/JS.
    //             let sim_variant = SimulationVariant::ThreeBodyMoon;
    //             // Request engine to be started on server.
    //             dom::console_log!("Requesting new Engine...");
    //             let request = request::Request::AddEngine(client_id, sim_variant);
    //             let request = TcpPackage::Request(request).to_bytes();
    //             ws.send_with_u8_array(&request).unwrap();
    //         }

    //         Response::AddedEngine(engine_id) => {
    //             dom::console_log!("New Engine confirmed. (id={:?})", engine_id);
    //             // Formulate state-query.
    //             let state_query = StateQuery::Between(0, STATE_BATCH_SIZE);
    //             dom::console_log!("Requesting States...");
    //             // Start sync-loop for this engine's states. TODO (?)
    //             let request = request::Request::GetUpdatedStates(engine_id, state_query);
    //             let request = TcpPackage::Request(request).to_bytes();
    //             ws.send_with_u8_array(&request).unwrap();
    //         }

    //         Response::StateVector(engine_id, states) => {
    //             dom::console_log!(
    //                 "Received {} states for engine {}...",
    //                 states.len(),
    //                 engine_id
    //             );
    //             // update state-id of most-recent sync
    //             let state_id = if states.len() == 0 {
    //                 0
    //             } else {
    //                 states[states.len() - 1].state_id // last update
    //             };

    //             // TODO do stuff with states (?)
    //         }

    //         Response::Empty => {}
    //         _ => todo!(),
    //     }
    // }

    // TcpPackage::Command(cmd) => match cmd {
    //     _ => todo!(),
    // },

    // TcpPackage::Request(req) => match req {
    //     _ => todo!(),
    // },
    // _ => todo!(),
    // }
    // 1
}
