#![allow(unreachable_code)]
use crate::config::preset;
use crate::config::EngineConfig;
use crate::integrator::euler;
use crate::integrator::runge_kutta;
use mxyz_engine_universe::integrator::Integrator;
use mxyz_engine_universe::integrator::IntegratorVariant;
use mxyz_engine_universe::preset::SimulationVariant;
use mxyz_engine_universe::state::State;
use mxyz_engine_universe::system::System;

pub trait Engine_ {
    fn engine_config(&self) -> &EngineConfig;
    fn engine_config_mut(&mut self) -> &mut EngineConfig;
    fn engine_id(&self) -> &usize;
    fn engine_states(&self) -> &Vec<State>;
    fn engine_states_mut(&mut self) -> &mut Vec<State>;
    fn add_engine_state(&mut self, state: State) {
        self.engine_states_mut().push(state);
    }
    /// Initialize state & engine-config.
    fn init(&mut self, sim_variant: Option<SimulationVariant>) {
        let initial_state = preset::initialize(sim_variant, &mut self.engine_config_mut());
        self.add_engine_state(initial_state);
    }
    fn run(&mut self) {
        // Run compute-loop for nr. of steps specified in engine configuration.
        let max_state_id = self.engine_config().step_id.1;
        for _ in 0..max_state_id {
            // Forward engine to next time-step.
            self.forward();
        }
    }
    fn forward(&mut self) {
        // Load current state.
        let state_id = self.engine_config().step_id.0;
        let current_state = &self.engine_states()[state_id];
        // Build trees / neighborhoods
        // TODO ...
        // Forward state to next time-step.
        let next_state = self.forward_state(&current_state);
        // Append new state to state-vector.
        self.add_engine_state(next_state);
        // Update step-id in engine-config.
        self.engine_config_mut().step_id.0 += 1;
    }
    fn forward_state(&self, state: &State) -> State;
    fn forward_system(&self, system: &System) -> System {
        let mut next_system: Option<System> = None;
        // Load list of integrators for system.
        let integrators = &system.integrators;

        let next_system = match integrators.len() {
            0 => system.clone(),
            _ => {
                let mut next_system = self.integrate_system(&integrators[0], &system);
                // Apply possible other integrators to system.
                for integrator in integrators[1..].iter() {
                    next_system = self.integrate_system(&integrator, &next_system);
                }
                next_system
            }
        };
        next_system
    }
    fn integrate_system(&self, integrator: &Integrator, system: &System) -> System {
        // Load interactions.
        let interactions = &integrator.interactions;
        // Match integration variant to find corresponding integrator function.
        let integrate = match integrator.variant {
            IntegratorVariant::EulerExplicit => euler::explicit::apply,
            IntegratorVariant::RungeKutta4 => runge_kutta::order_4::apply,
            _ => todo!("Integrator Variant"),
        };
        let system = todo!(); // integrate(system)
        system
    }
}

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
        let initial_state = crate::config::preset::initialize(sim_variant, &mut self.config);
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
        // Load current state.
        let current_state = &self.states[self.config.step_id.0];
        // Forward state to next time-step.
        let next_state = self.forward_state(&current_state);
        // Append new state to state-vector.
        self.states.push(next_state);
        // Update step-id in engine-config.
        self.config.step_id.0 += 1;
    }

    /// Forwards state to next time-step.
    pub fn forward_state(&self, state: &State) -> State {
        // TODO Build trees/neighborhoods for all systems.

        // Forward systems.
        let systems = state
            .systems
            .iter()
            .map(|sys| self.forward_system(sys))
            .collect();
        // Update state-id.
        let state_id = state.state_id + 1;
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
