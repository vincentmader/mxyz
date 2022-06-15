use super::renderer::Renderer;
use super::utils::dom;
use wasm_bindgen::prelude::*;
// use super::utils::dom::console;

// ----------------------------------------------------------------------------

#[wasm_bindgen]
/// Simulation Client: Renderer
pub struct SimulationRendererClient {
    renderer: Renderer,
    // TODO server -> bytestream -> client
}
#[wasm_bindgen]
impl SimulationRendererClient {
    /// Create new Simulation Renderer Client
    pub fn new() -> Self {
        let renderer = Renderer::new();
        SimulationRendererClient { renderer }
    }
    /// Initialize Renderer Client
    pub fn init(&mut self) {
        dom::set_panic_hook();
        self.renderer.init();
    }
    /// Run Renderer Client
    pub fn run(&mut self) {}
}

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
