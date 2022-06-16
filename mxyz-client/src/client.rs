use super::components::canvas::Canvas;
use super::config::RendererConfig;
use super::renderer::Renderer;
use super::utils::dom;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
// use super::utils::dom::console;

// ----------------------------------------------------------------------------

#[wasm_bindgen]
/// Simulation Client: Renderer
pub struct SimulationClientV1 {
    config: RendererConfig,
    renderer: Renderer,
    http_client: reqwest::Client,
    // TODO server -> bytestream -> client
}
#[wasm_bindgen]
impl SimulationClientV1 {
    /// Create new Simulation Renderer Client
    pub fn new() -> Self {
        let config = RendererConfig::new();
        let renderer = Renderer::new();
        let http_client = reqwest::Client::new();
        SimulationClientV1 {
            config,
            renderer,
            http_client,
        }
    }
    /// Initialize Renderer Client
    pub fn init(&mut self) {
        dom::set_panic_hook();
        self.renderer.init();
    }
    /// Run Renderer Client in Animation Loop
    pub fn run(mut self) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if self.config.frame_id.0 > self.config.frame_id.1 {
                let _ = f.borrow_mut().take();
                return;
            }
            self.step(); // the magic happens here
            dom::request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        dom::request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
    /// Forward Renderer to Next Time-Step
    pub fn step(&mut self) {
        // TODO this is just preliminary testing
        let i = self.config.frame_id.0;
        let r = i as f64;
        let phi = i as f64;
        let pos = (r * phi.cos(), r * phi.sin());
        let pos = (pos.0 + 500., pos.1 + 500.);

        let mut canvas = Canvas::new(0);
        canvas.set_stroke_style("white");
        canvas.set_fill_style("white");
        canvas.draw_circle(pos, 4., true);
        self.config.frame_id.0 += 1;

        self.sync();
    }
    /// Get New States from Engine on Server
    fn sync(&mut self) -> Result<(), reqwest::Error> {
        // let client = self.http_client;
        // let a = client.get("https://google.com").send()?;
        // let resp = client.get("http://httpbin.org/").send()?;
        // let a = reqwest::get("https://www.rust-lang.org").await;
        // dom::console::log(&format!("{:?}", a));
        Ok(())
        //     let content = reqwest::get("http://httpbin.org/range/26")
        //         .await?
        //         .text()
        //         .await?;
    }
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
