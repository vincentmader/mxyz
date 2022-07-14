use crate::utils::dom;
use mxyz_network::package::request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::state::StateQuery;
use mxyz_universe::system::sized::SizedSystemVariant;
use std::sync::mpsc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BinaryType::Arraybuffer;
use web_sys::WebSocket;
use web_sys::{ErrorEvent, MessageEvent};
// use crate::renderer::Renderer;

const STATE_BATCH_SIZE: i32 = 50;

/// Web-Socket Client
pub struct WebSocketClient {
    _client_id: usize,
    socket: WebSocket,
    tx_web_to_render: mpsc::Sender<Package>,
}
impl WebSocketClient {
    /// Creates new instance of Web Socket Client
    pub fn new(host: &str, port: u16, tx_web_to_render: mpsc::Sender<Package>) -> Self {
        let _client_id = 0; // TODO
        let address = format!("ws://{}:{}", host, port);
        let socket = WebSocket::new(&address).unwrap();
        WebSocketClient {
            _client_id,
            socket,
            tx_web_to_render,
        }
    }

    /// Initializes Web Socket Client
    pub fn init(&mut self) -> Result<(), JsValue> {
        dom::console_log!("Starting WebSocket Client...");
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
            dom::console_log!("TCP socket opened");
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
            dom::console_log!("ERROR Event: {:?}", e);
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

pub fn handle_arraybuffer(
    ws: &mut WebSocket,
    abuf: js_sys::ArrayBuffer,
    tx_web_to_render: std::sync::mpsc::Sender<Package>,
) {
    let array = js_sys::Uint8Array::new(&abuf);
    let _len = array.byte_length() as usize;
    let package = Package::from_bytes(array.to_vec());
    // console_log!("\nArraybuffer received {} bytes", len);
    // console_log!("\nArraybuffer received {} bytes -> {:?}", len, &package);
    handle_onmessage_package(ws, package, tx_web_to_render.clone());
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

pub fn handle_onmessage_package(
    ws: &mut WebSocket,
    package: Package,
    _tx_web_to_render: std::sync::mpsc::Sender<Package>,
) {
    match package {
        Package::Response(res) => {
            match res {
                Response::AddedClient(client_id) => {
                    // TODO load client-id into client struct field
                    // self.client_id = Some(client_id);

                    // TODO Get simulation-variant from HTML/JS.
                    let sim_variant = SimulationVariant::ThreeBodyMoon;

                    // Request engine to be started on server.
                    let request = request::Request::AddEngine(client_id, sim_variant);
                    let request = Package::Request(request).to_bytes();
                    ws.send_with_u8_array(&request).unwrap();
                }

                Response::AddedEngine(engine_id) => {
                    // Formulate state-query.
                    let state_query = StateQuery::Between(0, STATE_BATCH_SIZE);
                    // Start sync-loop for this engine's states.
                    let request = request::Request::GetUpdatedStates(engine_id, state_query);
                    let request = Package::Request(request).to_bytes();
                    ws.send_with_u8_array(&request).unwrap();
                }

                Response::StateVector(engine_id, state_vector) => {
                    // update state-id of last sync
                    let state_id = if state_vector.len() == 0 {
                        0
                    } else {
                        state_vector[state_vector.len() - 1].state_id // last update
                    };

                    use crate::renderer::components::canvas::Canvas;
                    let mut canvas = Canvas::new(0);
                    let cnv_dim = canvas.dimensions;
                    canvas.init();
                    canvas.set_fill_style("purple");
                    canvas.set_stroke_style("purple");

                    for state in state_vector.iter() {
                        // canvas.clear();

                        // let text = format!("state {}", state.state_id);
                        // let (x, y) = (50., 50.);
                        // canvas.fill_text(&text, x, y);

                        for system in state.systems.iter() {
                            match &system.variant {
                                SizedSystemVariant::EntitiesV1(system) => {
                                    for planet in system.entities.iter() {
                                        let pos = planet.position;
                                        let pos = (pos[0], pos[1]);
                                        let pos = (pos.0 * cnv_dim.0 / 2., pos.1 * cnv_dim.1 / 2.);
                                        let pos = (pos.0 + cnv_dim.0 / 2., pos.1 + cnv_dim.1 / 2.);
                                        let r = 1.;
                                        canvas.draw_circle(pos, r, true);
                                    }
                                }
                                _ => todo!(),
                            }
                        }
                    }

                    let state_query =
                        StateQuery::Between(state_id as i32, state_id as i32 + STATE_BATCH_SIZE);
                    let request = request::Request::GetUpdatedStates(engine_id, state_query);
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
            }
        }

        Package::Command(cmd) => match cmd {
            _ => todo!(),
        },

        Package::Request(req) => match req {
            _ => todo!(),
        },
    }
}
