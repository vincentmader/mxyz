use crate::utils::dom;
use mxyz_network::package::command::Command;
use mxyz_network::package::request;
use mxyz_network::package::request::Request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use mxyz_universe::preset::SimulationVariant;
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

#[wasm_bindgen]
pub struct WebSocketClient {
    address: String,
    socket: WebSocket,
}
#[wasm_bindgen]
impl WebSocketClient {
    /// Creates new instance of Web Socket Client
    pub fn new(host: &str, port: u16) -> Self {
        let address = format!("ws://{}:{}", host, port);
        let socket = WebSocket::new(&address).unwrap();
        WebSocketClient { address, socket }
    }

    /// Initializes Web Socket Client
    pub fn init(&mut self) -> Result<(), JsValue> {
        dom::console_log("Starting WebSocket Client...");
        self.socket.set_binary_type(Arraybuffer); // for small bin. msgs, like CBOR, Arraybuffer is more efficient than Blob handling
        self.create_onmessage_callback();
        self.create_onerror_callback();
        self.create_onopen_callback();
        Ok(())
    }

    // TODO
    pub fn get_states(&mut self) {
        let ws = &mut self.socket;
        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            console_log!("\nTCP socket opened"); // why no new line?

            // Get states. // TODO move to loop
            let state_id = 0; // TODO
            let request = request::Request::GetUpdatedStates(state_id);
            let request = Package::Request(request);
            let request = request.to_bytes();
            cloned_ws.send_with_u8_array(&request).unwrap();
            console_log!("get-state-vector binary msg successfully sent");
        }) as Box<dyn FnMut(JsValue)>);

        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
    }

    /// Creates OnOpen Callback.
    pub fn create_onopen_callback(&mut self) {
        let ws = &mut self.socket;

        let cloned_ws = ws.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            console_log!("TCP socket opened");

            // Add new client. TODO move to separate function (or match input)
            let request = request::Request::AddClient;
            let request = Package::Request(request);
            let request = request.to_bytes();
            match cloned_ws.send_with_u8_array(&request) {
                Ok(bytes) => {
                    // let package = bytes.
                }
                Err(err) => console_log!("ERROR sending message: {:?}", err),
            };
            console_log!("get-client binary msg successfully sent");

            let client_id = 0; // TODO

            // Add new engine. TODO move to separate function (or match input)
            let simulation_variant = SimulationVariant::ThreeBodyMoon;
            let request = request::Request::AddEngine(client_id, simulation_variant);
            let request = Package::Request(request);
            let request = request.to_bytes();
            cloned_ws.send_with_u8_array(&request).unwrap();
            console_log!("add-engine binary msg successfully sent");

            // Get states. // TODO move to loop
            let state_id = 0; // TODO
            let request = request::Request::GetUpdatedStates(state_id);
            let request = Package::Request(request);
            let request = request.to_bytes();
            cloned_ws.send_with_u8_array(&request).unwrap();
            console_log!("get-state-vector binary msg successfully sent");

            // // send off string message
            // match cloned_ws.send_with_str("ping") {
            //     Ok(_) => console_log!("message successfully sent"),
            //     Err(err) => console_log!("ERROR sending message: {:?}", err),
            // }
            // // send off binary message
            // match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
            //     Ok(_) => console_log!("binary message successfully sent"),
            //     Err(err) => console_log!("ERROR sending message: {:?}", err),
            //     _ => {}
            // }
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

        // Set error event handler on WebSocket.
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        // Forget the callback to keep it alive.
        onerror_callback.forget();
    }

    /// Creates OnMessage Callback.
    pub fn create_onmessage_callback(&mut self) {
        let ws = &mut self.socket;

        let cloned_ws = ws.clone();
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Handle ArrayBuffer.
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                // console_log!("message event, received arraybuffer: {:?}", abuf);
                let array = js_sys::Uint8Array::new(&abuf);
                let len = array.byte_length() as usize;
                // console_log!("Arraybuffer received {} bytes: {:?}", len, array.to_vec());
                console_log!("\nArraybuffer received {} bytes", len);

                let package = Package::from_bytes(array.to_vec());
                console_log!("Package: {:?}", &package);
                match package {
                    Package::Request(req) => match req {
                        Request::AddEngine(client_id, _simulation_variant) => { // Do nothing.
                        }
                        Request::GetUpdatedStates(_last_update) => { // Do nothing.
                        }
                        Request::RemoveEngine(_engine_id) => { // Do nothing.
                        }
                        Request::AddClient => { // Do nothing.
                        }
                    },
                    Package::Response(res) => match res {
                        Response::AddedEngine => {}
                        Response::AddedClient(client_id) => {
                            console_log!("Received client-id: {}", client_id);
                            // TODO
                        }
                        Response::StateVector(state_vector) => {
                            // TODO
                            println!("Received state vector {:?}", state_vector);
                            if state_vector.len() == 0 {
                                return;
                            }
                            console_log!(
                                "Received states: {:?} to {:?}",
                                state_vector[0].state_id,
                                state_vector[state_vector.len() - 1].state_id
                            );
                        }
                        Response::Empty => {}
                    },
                    Package::Command(cmd) => match cmd {
                        Command::SaveStatesToDatabase => todo!(),
                    },
                }

                // here you can for example use Serde Deserialize decode the message
                // for demo purposes we switch back to Blob-type and send off another binary message
                // cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
                // match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
                //     Ok(_) => console_log!("binary message successfully sent"),
                //     Err(err) => console_log!("ERROR sending message: {:?}", err),
                //     _ => {}
                // }

                // Handle Blob.
            } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
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
                })
                    as Box<dyn FnMut(web_sys::ProgressEvent)>);
                fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                fr.read_as_array_buffer(&blob).expect("blob not readable");
                onloadend_cb.forget();

            // Handle Text.
            } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                console_log!("message event, received Text: {:?}", txt);
            // Handle Other.
            } else {
                console_log!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        // Set message event handler on WebSocket.
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // Forget the callback to keep it alive.
        onmessage_callback.forget();
    }
}
