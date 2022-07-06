use crate::renderer::Renderer;
use crate::utils::dom;
use mxyz_network::package::command::Command;
use mxyz_network::package::request;
use mxyz_network::package::request::Request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use mxyz_universe::preset::SimulationVariant;
use std::sync::mpsc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BinaryType::Arraybuffer;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// #[wasm_bindgen]
pub struct WebSocketClient {
    client_id: usize,
    socket: WebSocket,
    tx_web_to_render: mpsc::Sender<Package>,
}
// #[wasm_bindgen]
impl WebSocketClient {
    /// Creates new instance of Web Socket Client
    pub fn new(host: &str, port: u16, tx_web_to_render: mpsc::Sender<Package>) -> Self {
        let client_id = 0; // TODO
        let address = format!("ws://{}:{}", host, port);
        let socket = WebSocket::new(&address).unwrap();
        WebSocketClient {
            client_id,
            socket,
            tx_web_to_render,
        }
    }

    /// Initializes Web Socket Client
    pub fn init(&mut self) -> Result<(), JsValue> {
        dom::console_log("Starting WebSocket Client...");
        self.socket.set_binary_type(Arraybuffer); // for small bin. msgs, like CBOR, Arraybuffer is more efficient than Blob handling
        self.create_onmessage_callback(self.tx_web_to_render.clone());
        self.create_onerror_callback();
        self.create_onopen_callback();
        Ok(())
    }

    /// Creates OnOpen Callback.
    pub fn create_onopen_callback(&mut self) {
        let ws = &mut self.socket;

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            console_log!("TCP socket opened");
            // Add new client.
            let request = Package::Request(request::Request::AddClient).to_bytes();
            cloned_ws.send_with_u8_array(&request).unwrap();
        }) as Box<dyn FnMut(JsValue)>);

        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    /// Creates OnError Callback.
    pub fn create_onerror_callback(&mut self) {
        let ws = &mut self.socket;

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            console_log!("ERROR Event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);

        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    }

    /// Creates OnMessage Callback.
    pub fn create_onmessage_callback(
        &mut self,
        tx_web_to_render: std::sync::mpsc::Sender<Package>,
    ) {
        let ws = &mut self.socket;

        let mut cloned_ws = ws.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Handle ArrayBuffer.
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                handle_arraybuffer(&mut cloned_ws, abuf, tx_web_to_render.clone());
            // Handle Blob.
            } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                handle_blob(&mut cloned_ws, blob);
            // Handle Text.
            } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                console_log!("message event, received Text: {:?}", txt);
            // Handle Other.
            } else {
                console_log!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }
}

pub fn handle_arraybuffer(
    ws: &mut WebSocket,
    abuf: js_sys::ArrayBuffer,
    tx_web_to_render: std::sync::mpsc::Sender<Package>,
) {
    let array = js_sys::Uint8Array::new(&abuf);
    let len = array.byte_length() as usize;
    let package = Package::from_bytes(array.to_vec());
    console_log!("\nArraybuffer received {} bytes", len);
    // console_log!("\nArraybuffer received {} bytes -> {:?}", len, &package);
    handle_onmessage_package(ws, package, tx_web_to_render.clone());
}

pub fn handle_blob(_ws: &mut WebSocket, blob: web_sys::Blob) {
    console_log!("message event, received blob: {:?}", blob);
    // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
    let fr = web_sys::FileReader::new().unwrap();
    let fr_c = fr.clone();

    // create onLoadEnd callback
    let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
        let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
        let len = array.byte_length() as usize;
        console_log!("Blob received {}bytes: {:?}", len, array.to_vec());
        // here you can for example use the received image/png data
    }) as Box<dyn FnMut(web_sys::ProgressEvent)>);

    fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
    fr.read_as_array_buffer(&blob).expect("blob not readable");
    onloadend_cb.forget();
}

pub fn handle_onmessage_package(
    ws: &mut WebSocket,
    package: Package,
    tx_web_to_render: std::sync::mpsc::Sender<Package>,
) {
    match package {
        Package::Response(res) => match res {
            Response::AddedClient(client_id) => {
                // TODO get simulation-variant from html/js
                let sim_variant = SimulationVariant::ThreeBodyMoon;

                // Request Engine to be started on Server.
                let request = request::Request::AddEngine(client_id, sim_variant);
                let request = Package::Request(request).to_bytes();
                ws.send_with_u8_array(&request).unwrap();
            }

            Response::AddedEngine(engine_id) => {
                // Start Sync-Loop for this Engine's States.
                let last_sync_id = 0; // should be fine like this
                let request = request::Request::GetUpdatedStates(engine_id, last_sync_id);
                let request = Package::Request(request).to_bytes();
                ws.send_with_u8_array(&request).unwrap();
            }

            Response::StateVector(engine_id, state_vector) => {
                // console_log!("Received states, {}", state_vector.len());
                let state_id = if state_vector.len() == 0 {
                    0
                } else {
                    console_log!(
                        "Received states: {:?} to {:?}",
                        state_vector[0].state_id,
                        state_vector[state_vector.len() - 1].state_id
                    );
                    state_vector[state_vector.len() - 1].state_id // last update
                };

                // let package = Package::StateVec(state_vec);
                // tx_web_to_render.send(package);

                let request = request::Request::GetUpdatedStates(engine_id, state_id);
                let request = Package::Request(request).to_bytes();
                ws.send_with_u8_array(&request).unwrap();

                // let (secs, nanos) = (1, 0);
                // let duration = core::time::Duration::new(secs, nanos);
                // wasm_timer::Delay::new(duration);
                // wasm_timer::sleep(duration);

                // use gloo_timers::callback::Timeout;
                // let timeout = Timeout::new(1_000, move || {
                // Do something after the one second timeout is up!
                // });
                // Since we don't plan on cancelling the timeout, call `forget`.
                // timeout.forget();
            }

            Response::Empty => {}
        },

        Package::Command(cmd) => match cmd {
            Command::SaveStatesToDatabase => todo!(),
        },

        Package::Request(req) => match req {
            _ => todo!("handle requests (?)"),
        },
    }
}
