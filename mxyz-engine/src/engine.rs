use super::config::EngineConfig;
use crate::tmp;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::state::State;
use mxyz_universe::system::System;
use serde::{Deserialize, Serialize};

/// MXYZ Simulation Engine
pub struct Engine {
    pub engine_id: usize,
    pub config: EngineConfig,
    pub states: Vec<State>,
}

impl Engine {
    /// Creates a new engine instance
    pub fn new(engine_id: usize) -> Self {
        let config = EngineConfig::new();
        let states = vec![];
        Engine {
            engine_id,
            config,
            states,
        }
    }

    /// Initializes state & config
    pub fn init(&mut self, simulation_variant: Option<SimulationVariant>) {
        println!("MXYZ-Engine: Initializing...");
        let mut initial_state = State::new();
        // initial_state.init(simulation_variant, &mut self.config);
        self.states.push(initial_state);
    }

    /// Runs engine
    pub fn run(&mut self) {
        println!("MXYZ-Engine: Running...");
        for _ in 0..self.config.step_id.1 {
            self.step();
        }
    }

    /// Forwards engine by one time-step
    pub fn step(&mut self) {
        /// Loads current State
        let current_state = &self.states[self.config.step_id.0];

        let mut next_state = State::new();
        next_state.state_id = current_state.state_id + 1;

        /// Creates "neighborhoods"
        // let _neighborhoods = tmp::prepare_neighborhoods(); // TODO get relevant neighbors/nodes

        /// Loops over systems & forwards each
        for system in &current_state.systems {
            let mut next_system = system.clone();
            /// Gets all Integrators for this System & loops over them
            for integrator in system.integrators.iter() {
                /// Gets all Interacting Systems for this Interaction
                // let other_ids = tmp::get_other_ids(&integrator, &current_state);
                let other_ids: Vec<usize> = todo!();
                /// Applies Interaction
                // integrator.step(&mut next_system, &current_state, &other_ids);
                match integrator {
                    _ => todo!(),
                }
            }
            next_state.systems.push(next_system);
        }

        // Forward to next time-step.
        // let next = current_state.next(&self.config, &self.states);
        self.states.push(next_state);
        self.config.step_id.0 += 1;
    }

    pub fn send(&self) {}

    pub fn receive(&self) {}

    pub fn get_unsaved_state_ids(&self) -> Vec<usize> {
        let a = self
            .states
            .iter()
            .filter(|state| {
                state.state_id
                    >= match self.config.last_export_step_id {
                        None => 0,
                        Some(e) => e + 1,
                    }
            })
            .map(|state| state.state_id)
            .collect();
        a
    }
}
