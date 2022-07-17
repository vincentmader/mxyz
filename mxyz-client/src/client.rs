use crate::renderer::Renderer;
use crate::utils::dom;
use crate::websocket::client::WebSocketClient;
use mxyz_config::ClientConfig;
use mxyz_network::package::request;
use mxyz_network::package::Package;
use mxyz_universe::state::StateQuery;
use wasm_bindgen::prelude::*;
// use mxyz_universe::system::SizedSystemVariant;
// use std::sync::mpsc;
// use std::sync::{Arc, Mutex};
// use std::cell::RefCell;
// use std::rc::Rc;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 1234;

#[wasm_bindgen]
/// Simulation-Client: Renderer
pub struct SimulationClientV1 {
    config: ClientConfig,
    renderer: Renderer,
    websocket: WebSocketClient,
}
#[wasm_bindgen]
impl SimulationClientV1 {
    /// Creates new Simulation-Renderer-Client
    pub fn new(client_id: usize) -> Self {
        let config = ClientConfig::new(client_id);
        let renderer = Renderer::new();
        let websocket = WebSocketClient::new(HOST, PORT);

        SimulationClientV1 {
            config,
            renderer,
            websocket,
        }
    }
    /// Initializes Renderer-Client
    pub fn init(&mut self, _simulation_variant: &str) {
        dom::set_panic_hook();

        self.renderer.init();
        self.websocket.init().unwrap();
    }
    /// Runs Renderer-Client in Animation Loop
    pub async fn run(self) {
        // loop {
        // TODO Get states.
        let state_id = 0;
        let engine_id = 0;
        let STATE_BATCH_SIZE = 10;
        // Request next states.
        // let ws = self.websocket.socket.clone();
        // dom::console_log!("Requesting States...");
        // let state_query = StateQuery::Between(state_id as i32, state_id as i32 + STATE_BATCH_SIZE);
        // let request = request::Request::GetUpdatedStates(engine_id, state_query);
        // dom::console_log!("{:?}", request);
        // let request = Package::Request(request).to_bytes();
        // ws.send_with_u8_array(&request).unwrap();

        // TODO Display states.
        // }
    }
}
//async fn loop_state_getter(
//    &self,
//    tx_web_to_render: Arc<Mutex<mpsc::Sender<Package>>>,
//) -> Result<(), JsValue> {
//    // let f = Rc::new(RefCell::new(None));
//    // let g = f.clone();
//    // let mut i = 0;
//    // *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
//    //     // let arc = arc.lock().unwrap();
//    //     // if arc.config.frame_id.0 > arc.config.frame_id.1 {
//    //     //     let _ = f.borrow_mut().take();
//    //     //     return;
//    //     // }
//    //     dom::console_log!("\t\t{:?} (state-getter-step)", i);
//    //     // TODO get states from here?
//    //     // std::thread::spawn(|| {});
//    //     // self.step(&tx); //
//    //     // self.step(); //
//    //     i += 1;
//    //     dom::request_animation_frame(f.borrow().as_ref().unwrap());
//    // }) as Box<dyn FnMut()>));
//    // dom::request_animation_frame(g.borrow().as_ref().unwrap());
//    Ok(())
//}

//async fn loop_state_renderer(
//    &self,
//    rx_web_to_render: Arc<Mutex<mpsc::Receiver<Package>>>,
//    // rx_web_to_render: mpsc::Receiver<Package>,
//    // arc: &Arc<Mutex<&mut SimulationClientV1>>
//) -> Result<(), JsValue> {
//    let _rx_web_to_render = rx_web_to_render.clone();
//    // let pkg = rx_web_to_render.lock().unwrap().recv().unwrap();

//    // use crate::renderer::components::canvas::Canvas;
//    // let mut canvas = Canvas::new(0);
//    // let cnv_dim = canvas.dimensions;
//    // canvas.init();
//    // canvas.set_fill_style("purple");
//    // canvas.set_stroke_style("purple");

//    //let f = Rc::new(RefCell::new(None));
//    //let g = f.clone();
//    //let mut i = 0;
//    //*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
//    //    // let pkg = rx_web_to_render.lock().unwrap().recv().unwrap();
//    //    //match pkg {
//    //    //    Package::StateVec(states) => {
//    //    //        //
//    //    //        // for state in states.iter() {
//    //    //        //     // canvas.clear();

//    //    //        //     // let text = format!("state {}", state.state_id);
//    //    //        //     // let (x, y) = (50., 50.);
//    //    //        //     // canvas.fill_text(&text, x, y);

//    //    //        //     for system in state.systems.iter() {
//    //    //        //         match &system.variant {
//    //    //        //             SizedSystemVariant::EntitiesV1(system) => {
//    //    //        //                 for planet in system.entities.iter() {
//    //    //        //                     let pos = planet.position;
//    //    //        //                     let pos = (pos[0], pos[1]);
//    //    //        //                     let pos = (pos.0 * cnv_dim.0 / 2., pos.1 * cnv_dim.1 / 2.);
//    //    //        //                     let pos = (pos.0 + cnv_dim.0 / 2., pos.1 + cnv_dim.1 / 2.);
//    //    //        //                     let r = 1.;
//    //    //        //                     canvas.draw_circle(pos, r, true);
//    //    //        //                 }
//    //    //        //             }
//    //    //        //             _ => todo!(),
//    //    //        //         }
//    //    //        //     }
//    //    //        // }

//    //    //        // let pkg = Package::StateVec(states);
//    //    //        // tx_web_to_render.send(pkg);

//    //    //        // let (secs, nanos) = (1, 0);
//    //    //        // let duration = core::time::Duration::new(secs, nanos);
//    //    //        // wasm_timer::Delay::new(duration);
//    //    //        // wasm_timer::sleep(duration);
//    //    //        // use gloo_timers::callback::Timeout;
//    //    //        // let timeout = Timeout::new(1_000, move || {
//    //    //        // Do something after the one second timeout is up!
//    //    //        // });
//    //    //        // Since we don't plan on cancelling the timeout, call `forget`.
//    //    //        // timeout.forget();
//    //    // }
//    //    //    _ => todo!(),
//    //    // }
//    //    // if self.config.frame_id.0 > self.config.frame_id.1 {
//    //    //     let _ = f.borrow_mut().take();
//    //    //     return;
//    //    // }

//    //    // let rx = rx_web_to_render.clone().lock().unwrap().recv().unwrap();
//    //    //match rx {
//    //    //    Package::Request(_) => {}
//    //    //    Package::Response(_) => {}
//    //    //    Package::Command(_) => {}
//    //    //    Package::StateVec(states) => {
//    //    //        use crate::renderer::components::canvas::Canvas;
//    //    //        let mut canvas = Canvas::new(0);
//    //    //        let cnv_dim = canvas.dimensions;
//    //    //        canvas.init();
//    //    //        canvas.set_fill_style("purple");
//    //    //        canvas.set_stroke_style("purple");

//    //    //        //
//    //    //        for state in states.iter() {
//    //    //            // canvas.clear();

//    //    //            // let text = format!("state {}", state.state_id);
//    //    //            // let (x, y) = (50., 50.);
//    //    //            // canvas.fill_text(&text, x, y);

//    //    //            for system in state.systems.iter() {
//    //    //                match &system.variant {
//    //    //                    SizedSystemVariant::EntitiesV1(system) => {
//    //    //                        for planet in system.entities.iter() {
//    //    //                            let pos = planet.position;
//    //    //                            let pos = (pos[0], pos[1]);
//    //    //                            let pos = (pos.0 * cnv_dim.0 / 2., pos.1 * cnv_dim.1 / 2.);
//    //    //                            let pos = (pos.0 + cnv_dim.0 / 2., pos.1 + cnv_dim.1 / 2.);
//    //    //                            let r = 1.;
//    //    //                            canvas.draw_circle(pos, r, true);
//    //    //                        }
//    //    //                    }
//    //    //                    _ => todo!(),
//    //    //                }
//    //    //            }
//    //    //        }
//    //    //    }
//    //    //}

//    //    dom::console_log!("{:?} (renderer-step)", i);
//    //    // std::thread::spawn(|| {});
//    //    // self.step(&tx); //
//    //    // self.step(); //
//    //    i += 1;
//    //    dom::request_animation_frame(f.borrow().as_ref().unwrap());
//    //}) as Box<dyn FnMut()>));
//    //dom::request_animation_frame(g.borrow().as_ref().unwrap());
//    Ok(())
//}

// /// Forwards Renderer to Next Time-Step
// pub fn step(&mut self) {
//     let _frame_id = self.config.frame_id.0;
//     // tmp::draw(i); // TODO create renderer with loop over systems & entities
//     self.config.frame_id.0 += 1;
// }

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
