pub mod config;
pub mod entity;
pub mod integrator;
pub mod interaction;
pub mod state;
pub mod system;
use config::simulation_variant::SimulationVariant;
use config::{preset, EngineConfig};
use entity::Entity;
use integrator::Integrator;
use state::UnsizedState;
use system::unsized_system::UnsizedSystem;

/// MXYZ Simulation-Engine
pub trait Engine {
    /// Initialize state & engine-config.
    fn init(&mut self, sim_variant: Option<SimulationVariant>) {
        let initial_state = preset::get_initial_state(sim_variant, self.engine_config_mut());
        self.add_engine_state(initial_state);
    }

    /// Forward engine to next time-step.
    fn forward_engine(&mut self) {
        let state = self.get_current_state();
        // Build trees / neighborhoods
        // TODO ...
        // Forward state to next time-step & append to state-vector.
        let state = self.forward_state(&state);
        self.add_engine_state(state);
        // Update step-id in engine-config.
        self.engine_config_mut().step_id.0 += 1;
    }

    /// Forward state to next time-step.
    fn forward_state(&self, state: &UnsizedState) -> UnsizedState;

    /// Forward system to next time-step, or clone (if no integrators are active).
    fn forward_or_clone_system(&self, system: (usize, &UnsizedSystem)) -> UnsizedSystem {
        let (system_id, system) = system;
        let system_cfg = self.engine_config().systems.get(&system_id).unwrap();
        let integrators = &system_cfg.integrators;
        // Loop over all integrators.
        let next_system = match integrators.len() {
            // If no integrators are active, then clone the system.
            0 => system.clone(),
            // If integrators are active, apply them to the system.
            _ => {
                let system = (system_id, system);
                // Apply first integrator.
                let integrator = integrators.get(0).expect("This should never fail.");
                let mut next_system = self.forward_system(integrator, system);
                // Apply possible other integrators to system.
                let integrators = integrators.get(1..).expect("This should never fail.");
                for integrator in integrators.iter() {
                    next_system = self.forward_system(integrator, system);
                }
                next_system
            }
        };
        next_system
    }

    /// Forward system to next time-step.
    fn forward_system(
        &self,
        integrator: &Integrator,
        system: (usize, &UnsizedSystem),
    ) -> UnsizedSystem;

    /// Forward entity to next time-step.
    fn forward_entity(
        &self,
        integrator: &Integrator,
        entity: ((usize, usize), &Box<dyn Entity>),
    ) -> Box<dyn Entity> {
        let state = self.get_current_state();
        let entity = integrator.forward_entity(entity, state, self.engine_config());
        entity
    }

    /// Get reference to engine-id.
    fn engine_id(&self) -> &usize;

    /// Get reference to state-vector.
    fn engine_states(&self) -> &Vec<UnsizedState>;

    /// Get mutable reference to state-vector.
    fn engine_states_mut(&mut self) -> &mut Vec<UnsizedState>;

    /// Get reference to engine-config.
    fn engine_config(&self) -> &EngineConfig;

    /// Get mutable reference to engine-config.
    fn engine_config_mut(&mut self) -> &mut EngineConfig;

    /// Add state to state-vector.
    fn add_engine_state(&mut self, state: UnsizedState) {
        self.engine_states_mut().push(state);
    }
    /// Get current state.
    fn get_current_state(&self) -> &UnsizedState {
        let state_id = self.engine_states().len() - 1;
        let state = self.engine_states().get(state_id).unwrap();
        state
    }
}
