#![allow(unreachable_code)]
use crate::config::EngineConfig;
use crate::integrator::euler;
use crate::integrator::runge_kutta;
use crate::preset;
use mxyz_engine_universe::integrator::Integrator;
use mxyz_engine_universe::integrator::IntegratorVariant;
use mxyz_engine_universe::preset::SimulationVariant;
use mxyz_engine_universe::state::State;
use mxyz_engine_universe::system::System;

// ============================================================================

/// MXYZ Simulation-Engine
pub struct Engine {
    pub engine_id: usize,
    pub config: EngineConfig,
    pub states: Vec<State>,
}
impl Engine {
    /// Creates a new engine instance.
    pub fn new(engine_id: usize) -> Self {
        // Create new engine-config.
        let config = EngineConfig::new();
        // Initialize empty state-vector.
        let states = vec![];

        Engine {
            engine_id,
            config,
            states,
        }
    }

    /// Initializes state & engine-config.
    pub fn init(&mut self, sim_variant: Option<SimulationVariant>) {
        let initial_state = preset::initialize(sim_variant, &mut self.config);
        self.states.push(initial_state);
    }

    /// Runs engine.
    pub fn run(&mut self) {
        // Run compute-loop for nr. of steps specified in engine configuration.
        for _ in 0..self.config.step_id.1 {
            // Forward engine to next time-step.
            self.forward();
        }
    }

    /// Forwards engine to next time-step.
    pub fn forward(&mut self) {
        // Forward state to next time-step.
        let next_state = self.forward_state();
        // Append new state to state-vector.
        self.states.push(next_state);
        // Update step-id in engine-config.
        self.config.step_id.0 += 1;
    }

    /// Forwards state to next time-step.
    pub fn forward_state(&self) -> State {
        // Load current state.
        let current_state = &self.states[self.config.step_id.0];
        // Update state-id.
        let state_id = current_state.state_id + 1;

        // TODO Build trees/neighborhoods for all systems.

        // Forward systems.
        let systems = current_state
            .systems
            .iter()
            .map(|sys| self.forward_system(sys))
            .collect();
        // Return next state.
        State { state_id, systems }
    }

    /// Forwards system to next time-step.
    pub fn forward_system(&self, system: &System) -> System {
        let mut next_system: System;
        // Load list of integrators for system.
        let integrators = &system.integrators;
        // Handle case of at least one integrator. (basically always)
        if integrators.len() > 0 {
            // Apply first integrator to system.
            next_system = self.integrate_system(&integrators[0], &system);
            // Loop over other integrators.
            for integrator in integrators[1..].iter() {
                // Apply possible other integrators to system.
                next_system = self.integrate_system(&integrator, &next_system);
            }
        // Handle case of empty integrator-vector.
        // - should not occur in practice, but might!
        } else {
            next_system = system.clone();
        }
        next_system
    }

    /// Applies integration scheme to system.
    fn integrate_system(&self, integrator: &Integrator, system: &System) -> System {
        // Load interactions.
        let interactions = &integrator.interactions;
        // Match integration variant to find corresponding integrator function.
        let integrate = match integrator.variant {
            IntegratorVariant::EulerExplicit => euler::explicit::apply,
            IntegratorVariant::RungeKutta4 => runge_kutta::order_4::apply,
            _ => todo!("Integrator Variant"),
        };
        // Return integrated system.
        integrate(&system, &self.states, interactions)
    }

    /// Gets state-ids of states not yet saved to database.
    pub fn get_unsaved_state_ids(&self) -> Vec<usize> {
        self.states
            .iter()
            .filter(|state| {
                state.state_id
                    >= match self.config.last_export_step_id {
                        // If last-export-id is None, load all states since 0.
                        None => 0,
                        // If not None, load states since last-export-id + 1.
                        Some(e) => e + 1,
                    }
            })
            .map(|state| state.state_id)
            .collect()
    }
}
