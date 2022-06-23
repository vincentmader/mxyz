use super::components::canvas::Canvas;
use super::config::ClientConfig;
use super::renderer::Renderer;
use super::utils::dom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use super::utils::dom::console;

// pub struct Client {
//     pub user_id: usize,
//     pub topics: Vec<String>,
//     pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
// }

// ----------------------------------------------------------------------------

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    config: ClientConfig,
    renderer: Renderer,
    http_client: reqwest::Client,
    // TODO server -> bytestream -> client
}
#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    pub fn new() -> Self {
        let config = ClientConfig::new();
        let renderer = Renderer::new();
        let http_client = reqwest::Client::new();
        SimulationClientV1 {
            config,
            renderer,
            http_client,
        }
    }
    /// Initializes Renderer-Client
    pub fn init(&mut self, sim_id: &str) {
        dom::set_panic_hook();

        dom::console_log(sim_id);
        self.renderer.init();
    }
    /// Runs Renderer-Client in Animation Loop
    pub async fn run(mut self) -> Result<(), JsValue> {
        // TODO test get-request to server
        // - TCP get-requests (bytestream? -> decode)
        // - move inside animation loop (async?)
        let client = reqwest::Client::new();
        let mut stream = client
            .get("http://127.0.0.1:8000/test_db")
            .send()
            .await
            .unwrap();
        let body = stream.bytes().await.unwrap();
        dom::console_log(&format!("{:?}", body));

        start_websocket().unwrap();

        // ANIMATION LOOP
        // todo: move to utils/dom/mod.rs (?)
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if self.config.frame_id.0 > self.config.frame_id.1 {
                let _ = f.borrow_mut().take();
                return;
            }
            self.step(); //
                         // let client = reqwest::Client::new();
                         // let mut stream = client
                         //     .get("http://127.0.0.1:8000/test_db")
                         //     .send()
                         //     .await
                         //     .unwrap();
                         // let body = stream.bytes().await.unwrap();
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

        self.sync();
    }
    /// Gets New States from Engine on Server
    async fn sync(&mut self) -> Result<(), reqwest::Error> {
        let a = ":(";
        // let a = self.http_client.get("https://google.com").send().await;
        // let client = self.http_client;
        // let a = client.get("https://google.com").send()?;
        // let resp = client.get("http://httpbin.org/").send()?;
        // let a = reqwest::get("https://www.rust-lang.org").await;
        dom::console_log(&format!("{:?}", a));
        Ok(())
        //     let content = reqwest::get("http://httpbin.org/range/26")
        //         .await?
        //         .text()
        //         .await?;
    }
}

// TODO
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

// ============================================================================

// ----------------------------------------------------------------------------
// use mxyz_engine::state::preset::SimulationId;
// use mxyz_engine::system::System;
// use mxyz_engine::Engine;

// #[wasm_bindgen]
// /// Simulation Client: Compute + Renderer
// pub struct SimulationComputeClient {
//     // engine: Engine,
//     renderer: Renderer,
// }
// #[wasm_bindgen]
// impl SimulationComputeClient {
//     /// Create new Simulation Compute Client
//     pub fn new() -> Self {
//         let renderer = Renderer::new();
//         // let engine = Engine::new();
//         SimulationComputeClient {
//             renderer,
//             // engine
//         }
//     }
//     /// Initialize Compute Client
//     pub fn init(&mut self) {
//         // dom::set_panic_hook();
//         // let sim_id = SimulationId::Foo;
//         // self.engine.init(&Some(sim_id));
//         // self.renderer.init();
//     }
//     /// Run Compute Client
//     pub fn run(&mut self) {
//         for _ in 0..13000 {
//             // self.engine.step();
//             // self.render();
//             // crate::animation_loop::run();
//         }
//     }
//     pub fn render(&mut self) {
//         // let canvas = &mut self.renderer.canvases[0];
//         // canvas.clear();

//         // let config = &self.engine.config;
//         // let states = &self.engine.states;
//         // let step_id = config.step_id;
//         // let current_state = &states[step_id];
//         // for system in current_state.systems.iter() {
//         //     match system {
//         //         System::PhysicalBodies(sys) => {
//         //             for body in sys.entities.iter() {
//         //                 let mass = body.mass;
//         //                 let pos = body.position;
//         //                 let vel = body.velocity;
//         //                 // let a = format!("{:?}", pos);
//         //                 // console::log(&a);

//         //                 canvas.set_stroke_style("white");
//         //                 canvas.set_fill_style("white");
//         //                 let r = 0.01;
//         //                 canvas.draw_circle((pos[0], pos[1]), r, true);
//         //             }
//         //         }
//         //         System::ForceField(sys) => {}
//         //         _ => {}
//         //     }
//         // }
//         // let a = format!("{}", config.step_id);
//         // crate::utils::dom::console::log(&a);
//     }
// }

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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
pub fn start_websocket() -> Result<(), JsValue> {
    let address = "wss://echo.websocket.events";
    let ws = WebSocket::new(address)?;

    // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    // create callback
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        // Handle difference Text/Binary,...
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            console_log!("message event, received arraybuffer: {:?}", abuf);

            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            console_log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());

            // here you can for example use Serde Deserialize decode the message
            // for demo purposes we switch back to Blob-type and send off another binary message
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
            match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
                //     Ok(_) => console_log!("binary message successfully sent"),
                //     Err(err) => console_log!("error sending message: {:?}", err),
                _ => {}
            }
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
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            console_log!("message event, received Text: {:?}", txt);
        } else {
            console_log!("message event, received Unknown: {:?}", e.data());
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    // set message event handler on WebSocket
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();

    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        console_log!("error event: {:?}", e);
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::wrap(Box::new(move |_| {
        console_log!("socket opened");
        // match cloned_ws.send_with_str("ping") {
        //     Ok(_) => console_log!("message successfully sent"),
        //     Err(err) => console_log!("error sending message: {:?}", err),
        // }
        // send off binary message
        match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
            // Ok(_) => console_log!("binary message successfully sent"),
            // Err(err) => console_log!("error sending message: {:?}", err),
            _ => {}
        }
    }) as Box<dyn FnMut(JsValue)>);
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}
