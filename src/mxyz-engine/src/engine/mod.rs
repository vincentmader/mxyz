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

/// MXYZ Simulation-Engine
pub trait Engine {
    fn engine_id(&self) -> &usize;
    fn engine_states(&self) -> &Vec<State>;
    fn engine_states_mut(&mut self) -> &mut Vec<State>;
    fn engine_config(&self) -> &EngineConfig;
    fn engine_config_mut(&mut self) -> &mut EngineConfig;
    fn add_engine_state(&mut self, state: State) {
        self.engine_states_mut().push(state);
    }
    /// Initialize state & engine-config.
    fn init(&mut self, sim_variant: Option<SimulationVariant>) {
        let initial_state = preset::initialize(sim_variant, &mut self.engine_config_mut());
        self.add_engine_state(initial_state);
    }
    /// Run engine.
    fn run(&mut self) {
        // Run compute-loop for nr. of steps specified in engine configuration.
        let max_state_id = self.engine_config().step_id.1;
        for _ in 0..max_state_id {
            // Forward engine to next time-step.
            self.forward();
        }
    }
    /// Forward Engine to next time-step.
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
    /// Forward state to next time-step.
    fn forward_state(&self, state: &State) -> State;
    /// Forward system to next time-step.
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
    /// Apply integration scheme to system.
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
