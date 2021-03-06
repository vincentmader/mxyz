use crate::utils::dom;
use mxyz_network::package::request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::state::SizedState;
use mxyz_universe::state::StateQuery;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BinaryType::Arraybuffer;
use web_sys::WebSocket;
use web_sys::{ErrorEvent, MessageEvent};
// use crate::renderer::Renderer;
// use mxyz_universe::system::SizedSystemVariant;
// use std::sync::mpsc;
// use std::sync::{Arc, Mutex};

const STATE_BATCH_SIZE: i32 = 50;

/// Web-Socket Client
pub struct WebSocketClient {
    _client_id: usize,
    pub socket: WebSocket,
}
impl WebSocketClient {
    /// Creates new instance of Web Socket Client
    pub fn new(host: &str, port: u16) -> Self {
        let _client_id = 0; // TODO
        let address = format!("ws://{}:{}", host, port);
        let socket = WebSocket::new(&address).unwrap();
        WebSocketClient { _client_id, socket }
    }

    /// Initializes Web Socket Client
    pub fn init(&mut self) -> Result<(), JsValue> {
        dom::console_log!("Starting WebSocket Client...");
        self.socket.set_binary_type(Arraybuffer); // for small bin. msgs, like CBOR, Arraybuffer is more efficient than Blob handling
        self.create_onmessage_callback();
        self.create_onerror_callback();
        self.create_onopen_callback();
        Ok(())
    }

    /// Creates OnOpen Callback.
    pub fn create_onopen_callback(&mut self) {
        let ws = &mut self.socket;

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            dom::console_log!("TCP socket opened.");
            // Add new client.
            let request = Package::Request(request::Request::AddClient).to_bytes();
            dom::console_log!("Requesting new Client...");
            cloned_ws.send_with_u8_array(&request).unwrap();
        }) as Box<dyn FnMut(JsValue)>);

        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    /// Creates OnError Callback.
    pub fn create_onerror_callback(&mut self) {
        let ws = &mut self.socket;

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            dom::console_log!("ERROR Event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);

        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    }

    /// Creates OnMessage Callback.
    pub fn create_onmessage_callback(&mut self) {
        let ws = &mut self.socket;

        let mut cloned_ws = ws.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Handle ArrayBuffer.
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                handle_arraybuffer(&mut cloned_ws, abuf);
            // Handle Blob.
            } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                handle_blob(&mut cloned_ws, blob);
            // Handle Text.
            } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                dom::console_log!("message event, received Text: {:?}", txt);
            // Handle Other.
            } else {
                dom::console_log!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }
}

pub fn handle_arraybuffer(ws: &mut WebSocket, abuf: js_sys::ArrayBuffer) {
    let array = js_sys::Uint8Array::new(&abuf);
    let _len = array.byte_length() as usize;
    let package = Package::from_bytes(array.to_vec());
    // console_log!("\nArraybuffer received {} bytes", len);
    // console_log!("\nArraybuffer received {} bytes -> {:?}", len, &package);
    handle_onmessage_package(ws, package);
}

pub fn handle_blob(_ws: &mut WebSocket, blob: web_sys::Blob) {
    dom::console_log!("UNHANDLED message event, received blob: {:?}", blob);
    // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
    let fr = web_sys::FileReader::new().unwrap();
    let fr_c = fr.clone();

    // create onLoadEnd callback
    let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        let len = array.byte_length() as usize;
        dom::console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
        // here you can for example use the received image/png data
    }) as Box<dyn FnMut(web_sys::ProgressEvent)>);

    fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    fr.read_as_array_buffer(&blob).expect("blob not readable");
    onloadend_cb.forget();
}

pub fn handle_onmessage_package(ws: &mut WebSocket, package: Package) {
    match package {
        Package::Response(res) => {
            match res {
                Response::AddedClient(client_id) => {
                    dom::console_log!("New Client confirmed. (id={:?})", client_id);
                    // TODO load client-id into client struct field
                    // self.client_id = Some(client_id);

                    // TODO Get simulation-variant from HTML/JS.
                    let sim_variant = SimulationVariant::ThreeBodyMoon;

                    // Request engine to be started on server.
                    dom::console_log!("Requesting new Engine...");
                    let request = request::Request::AddEngine(client_id, sim_variant);
                    let request = Package::Request(request).to_bytes();
                    ws.send_with_u8_array(&request).unwrap();
                }

                Response::AddedEngine(engine_id) => {
                    dom::console_log!("New Engine confirmed. (id={:?})", engine_id);
                    // Formulate state-query.
                    let state_query = StateQuery::Between(0, STATE_BATCH_SIZE);
                    dom::console_log!("Requesting States...");
                    // Start sync-loop for this engine's states.
                    let request = request::Request::GetUpdatedStates(engine_id, state_query);
                    let request = Package::Request(request).to_bytes();
                    ws.send_with_u8_array(&request).unwrap();
                }

                Response::StateVector(engine_id, states) => {
                    dom::console_log!(
                        "Received {} states for engine {}...",
                        states.len(),
                        engine_id
                    );
                    // update state-id of most-recent sync
                    let state_id = if states.len() == 0 {
                        0
                    } else {
                        states[states.len() - 1].state_id // last update
                    };

                    // TODO do stuff with states (?)
                }

                Response::Empty => {}
                _ => todo!(),
            }
        }

        Package::Command(cmd) => match cmd {
            _ => todo!(),
        },

        Package::Request(req) => match req {
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn draw_states(states: Vec<SizedState>) {}
