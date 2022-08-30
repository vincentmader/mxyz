use crate::renderer::components::canvas::Canvas;
use crate::utils::dom;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use mxyz_engine::state::SizedState;
use mxyz_engine::state::StateQuery;
use mxyz_engine::system::SizedSystemVariant;
use mxyz_network::tcp_pkg::request;
use mxyz_network::tcp_pkg::request::Request;
use mxyz_network::tcp_pkg::response::Response;
use mxyz_network::tcp_pkg::TcpPackage;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BinaryType::Arraybuffer;
use web_sys::WebSocket;
use web_sys::{ErrorEvent, MessageEvent};

const STATE_BATCH_SIZE: i32 = 500;
const PARTICLE_RADIUS: f64 = 10.;

/// Web-Socket TCP Client
pub struct WebSocketClient {
    pub socket: WebSocket,
}
impl WebSocketClient {
    /// Create new instance of Client.
    pub fn new(host: &str, port: u16) -> Self {
        let address = format!("ws://{}:{}", host, port);
        let socket = WebSocket::new(&address).unwrap();
        WebSocketClient { socket }
    }

    /// Initialize Client, i.e. Create Callbacks:
    /// - on-open
    /// - on-message
    /// - on-error
    pub fn init(&mut self) -> Result<(), JsValue> {
        self.socket.set_binary_type(Arraybuffer);
        self.create_onmessage_callback();
        self.create_onerror_callback();
        self.create_onopen_callback();
        Ok(())
    }

    /// Create TCP OnOpen Callback.
    pub fn create_onopen_callback(&mut self) {
        let ws = &mut self.socket;
        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            dom::console_log!("TCP socket opened. Requesting new Client...");
            // Define Request: Add new Client on Server.
            let req = request::Request::AddClient; // TODO [#1] include page-id as well?
            let req = TcpPackage::Request(req);
            let req = req.to_bytes();
            // Send Request via TCP Stream to Server.
            cloned_ws.send_with_u8_array(&req).unwrap();
        }) as Box<dyn FnMut(JsValue)>);
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    /// Create TCP OnError Callback.
    pub fn create_onerror_callback(&mut self) {
        let ws = &mut self.socket;
        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            dom::console_log!("TCP ERROR Event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    }

    /// Create TCP OnMessage Callback.
    pub fn create_onmessage_callback(&mut self) {
        let ws = &mut self.socket;
        let mut cloned_ws = ws.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Handle ArrayBuffer Packages.
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let array = js_sys::Uint8Array::new(&abuf);
                let package = TcpPackage::from_bytes(array.to_vec());
                handle_package(&mut cloned_ws, package);
            } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                dom::console_log!("TCP event, received Blob: {:?}", blob);
            } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                dom::console_log!("TCP event, received Text: {:?}", txt);
            } else {
                dom::console_log!("TCP event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();
    }
}

// =============================================================================

/// Handle Incoming Tcp Package.
///
/// Different Variants:
/// - Request    (not relevant)
/// - Response   (relevant!)
pub fn handle_package(ws: &mut WebSocket, package: TcpPackage) {
    match package {
        TcpPackage::Request(req) => handle_request(ws, req),
        TcpPackage::Response(res) => handle_response(ws, res),
    }
}
/// Handle Incoming Request.
pub fn handle_request(_ws: &mut WebSocket, request: Request) {
    match request {
        _ => {}
    };
}
/// Handle Incoming Response.
pub fn handle_response(ws: &mut WebSocket, response: Response) {
    match response {
        Response::AddedClient(client_id) => handle_added_client(ws, client_id),
        Response::AddedEngine(engine_id) => handle_added_engine(ws, engine_id),
        Response::StateVector(engine_id, query, states) => {
            handle_received_states(ws, engine_id, query, states)
        }
        Response::Empty => {}
    }
}

// =============================================================================

/// Handle Response: Added Client.
pub fn handle_added_client(ws: &mut WebSocket, client_id: usize) {
    dom::console_log!("Client ({:?}) confirmed. Requesting Engine...", client_id);
    // TODO [#1] Get simulation-variant from HTML/JS.
    let sim_variant = SimulationVariant::ThreeBodyMoon;
    // Request Engine to be started on Server.
    let request = request::Request::AddEngine(client_id, sim_variant);
    let request = TcpPackage::Request(request).to_bytes();
    ws.send_with_u8_array(&request).unwrap();
}

/// Handle Response: Added Engine.
pub fn handle_added_engine(ws: &mut WebSocket, engine_id: usize) {
    dom::console_log!("Engine ({:?}) confirmed. Requesting States...", engine_id);
    // Formulate state-query.
    let state_query = StateQuery::Between(0, STATE_BATCH_SIZE);
    // Start sync-loop for this Engine's States. TODO (?)
    let request = request::Request::GetUpdatedStates(engine_id, state_query);
    let request = TcpPackage::Request(request).to_bytes();
    ws.send_with_u8_array(&request).unwrap();
}

/// Handle Response: Received States.
pub fn handle_received_states(
    ws: &mut WebSocket,
    engine_id: usize,
    query: StateQuery,
    states: Vec<SizedState>,
) {
    dom::console_log!("Received {} States for Engine {}.", states.len(), engine_id);

    let nr_of_states = states.len();
    let received_states = nr_of_states > 0;
    let state_query = match received_states {
        true => {
            let state_id = states.get(nr_of_states - 1).unwrap().state_id as i32;
            let state_query = StateQuery::BatchSince(STATE_BATCH_SIZE, state_id);
            dom::console_log!("{:?}", state_query);
            state_query
        }
        false => query,
    };

    // Ask for new States once again.
    let request = request::Request::GetUpdatedStates(engine_id, state_query);
    let request = TcpPackage::Request(request).to_bytes();
    ws.send_with_u8_array(&request).unwrap();

    // TODO do stuff with states (?)
    let mut finished_with_last_render = false;
    // if states.len() > 0 && finished_with_last_render {
    if states.len() > 0 {
        // finished_with_last_render = false;
        draw_states(states, &mut finished_with_last_render).unwrap();
    }
}

// =============================================================================

pub fn draw_states(
    states: Vec<SizedState>,
    finished_with_last_render: &mut bool,
) -> Result<(), JsValue> {
    let states = Arc::new(Mutex::new(states));

    let mut canvas = Canvas::new(0);
    canvas.init();
    canvas.set_fill_style("purple");
    canvas.set_stroke_style("purple");

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut i = 0;
    let mut nr_of_states = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i >= STATE_BATCH_SIZE {
            let _ = f.borrow_mut().take();
            return;
        }

        let states = states.clone();
        let states = states.lock().unwrap();
        if nr_of_states == states.len() {
            canvas.clear();
        } else {
            nr_of_states = states.len();
        }
        // dom::console_log!("{} / {}", i, nr_of_states);
        // dom::console_log!("{} < {}", i, STATE_BATCH_SIZE);

        // dom::console_log!("{}", i);
        let state = states.get(i as usize).unwrap(); // TODO
        for system in state.systems.iter() {
            match &system.variant {
                SizedSystemVariant::EntitiesV1(sys) => {
                    //
                    for entity in sys.entities.iter() {
                        let _mass = entity.mass;
                        let pos = entity.position;
                        let _vel = entity.velocity;

                        let _cnv_dim = canvas.dimensions;
                        canvas.draw_circle([pos[0], pos[1]], PARTICLE_RADIUS, true);
                    }
                }
                _ => {}
            }
        }

        i += 1;
        // Schedule ourself for another requestAnimationFrame callback.
        dom::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    dom::request_animation_frame(g.borrow().as_ref().unwrap());

    *finished_with_last_render = true;
    Ok(())
}
