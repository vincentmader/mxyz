use super::EngineRunner;
use crate::renderer::engine_renderer::EngineRenderer;
use crate::utils::dom;
use mxyz_client_engine::SimulationEngineV1;
use mxyz_engine::Engine;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const NR_OF_FORWARDS_BTW_RENDER: usize = 1;

/// Simulation-Client v2
/// - Compute States
/// - Render to Canvas (Animation Loop).
pub struct EngineRunnerClientV1 {}

impl EngineRunnerClientV1 {
    /// Create new Simulation-Compute-Client.
    pub fn new() -> Self {
        EngineRunnerClientV1 {}
    }
}
impl EngineRunner for EngineRunnerClientV1 {
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

        let mut renderer = EngineRenderer::new();
        renderer.init();

        let n = usize::MAX; // TODO make customizable (why not with config.step-id?)

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut state_id = 0;
        *g.borrow_mut() = Some(Closure::new(move || {
            if state_id > n {
                let _ = f.borrow_mut().take();
                return;
            }

            engine.forward_engine();
            if state_id % NR_OF_FORWARDS_BTW_RENDER == 0 {
                let state = engine.states.get(state_id).unwrap();
                renderer.display_state(state);
            }

            state_id += 1;
            // Schedule ourself for another requestAnimationFrame callback.
            dom::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        dom::request_animation_frame(g.borrow().as_ref().unwrap());
    }
}
