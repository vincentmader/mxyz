use super::EngineRunner;
use crate::renderer::Renderer;
use crate::utils::dom;
use mxyz_client_engine::SimulationEngineV1;
use mxyz_engine::engine::Engine;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Simulation-Client v2
/// - Compute States
/// - Render to Canvas (Animation Loop).
pub struct SimulationClientV1Compute {}

impl SimulationClientV1Compute {
    /// Create new Simulation-Compute-Client.
    pub fn new() -> Self {
        SimulationClientV1Compute {}
    }
}
impl EngineRunner for SimulationClientV1Compute {
    /// Initialize Compute-Client.
    /// - ... page-id
    /// - ... panic hook
    fn init(&mut self, category: &str, simulation_variant: &str) {
        let page_id = (category, simulation_variant);
        dom::console_log!("page-id: {:?}", page_id);
        // ...
        dom::set_panic_hook();

        let simulation_variant = Some(simulation_variant.into());
        let mut engine = mxyz_client_engine::SimulationEngineV1::new(0);
        engine.init(simulation_variant);

        let mut renderer = Renderer::new();
        renderer.init();

        let n = usize::MAX;
        // for state_id in 0..n {
        //     // dom::console_log!("{:?}", state);
        // }

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut state_id = 0;
        *g.borrow_mut() = Some(Closure::new(move || {
            if state_id > n {
                // dom::body().set_text_content(Some("All done!"));

                // Drop our handle to this closure so that it will get cleaned
                // up once we return.
                let _ = f.borrow_mut().take();
                return;
            }

            engine.forward_engine();
            let state = engine.states.get(state_id).unwrap();
            renderer.draw_state(state);

            state_id += 1;
            // Schedule ourself for another requestAnimationFrame callback.
            dom::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        dom::request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
