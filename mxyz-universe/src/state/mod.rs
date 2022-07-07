use crate::preset::SimulationVariant;
use crate::system::System;
use serde::{Deserialize, Serialize};

/// State
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub state_id: usize,
    pub systems: Vec<System>,
}
impl State {
    /// Creates new instance of State Structure
    pub fn new() -> Self {
        let state_id = 0;
        let systems = vec![];
        State { state_id, systems }
    }

    /// Initializes State & configuration
    pub fn init(
        &mut self,
        simulation_variant: Option<SimulationVariant>,
        // config: &mut EngineConfig,
    ) {
        // self.systems = preset::initialize(simulation_variant, config);
    }

    // /// Forwards State
    // pub fn next(&self, config: &EngineConfig, states: &Vec<State>) -> State {
    //     let mut next_state = State::new();
    //     next_state.state_id = self.state_id + 1;

    //     /// Loads current State
    //     let current_state = &states[config.step_id.0];

    //     /// Creates "neighborhoods"
    //     let _neighborhoods = tmp::prepare_neighborhoods(); // TODO get relevant neighbors/nodes

    //     /// Loops over systems & forwards each
    //     for system in &current_state.systems {
    //         let mut next_system = system.clone();
    //         /// Gets all Integrators for this System & loops over them
    //         let integrators = tmp::get_integrators(&system, &config).unwrap();
    //         for integrator in integrators {
    //             /// Gets all Interacting Systems for this Interaction
    //             let other_ids = tmp::get_other_ids(&integrator, &current_state);
    //             /// Applies Interaction
    //             integrator.step(&mut next_system, &current_state, &other_ids);
    //         }
    //         next_state.systems.push(next_system);
    //     }
    //     next_state
    // }
}
