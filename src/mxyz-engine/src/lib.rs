pub mod config;
pub mod entity;
pub mod integrator;
pub mod interaction;
pub mod neighborhoods;
pub mod state;
pub mod system;
use config::simulation_variant::SimulationVariant;
use config::{preset, EngineConfig};
use entity::Entity;
use integrator::Integrator;
use neighborhoods::Neighborhoods;
use state::UnsizedState;
use system::unsized_system::UnsizedSystem;

/// MXYZ Simulation-Engine
pub trait Engine {
    /// Initialize state & engine-config.
    fn init(&mut self, sim_variant: Option<SimulationVariant>) {
        // Initialize state.
        let initial_state = preset::get_initial_state(sim_variant, self.mut_engine_config());
        // Add state to engine state-vector.
        self.add_engine_state(initial_state);
    }

    fn run(&mut self);

    /// Forward engine to next time-step.
    fn forward_engine(&mut self) {
        // Get current state.
        let state = self.get_current_state();
        // Build entity-neighborhoods from current state.
        let neighborhoods = Neighborhoods::from(state);
        // Forward state to next time-step.
        let state = self.forward_state(&state, &neighborhoods);
        // Append next state to engine state-vector.
        self.add_engine_state(state);
        // Update step-id in engine-config.
        self.mut_engine_config().step_id.0 += 1;
    }

    /// Forward state to next time-step.
    fn forward_state(&self, state: &UnsizedState, neighborhoods: &Neighborhoods) -> UnsizedState;

    /// Forward system to next time-step, or clone (if no integrators are active).
    fn forward_or_clone_system(
        &self,
        system: (usize, &UnsizedSystem),
        neighborhoods: &Neighborhoods,
    ) -> UnsizedSystem {
        let (system_id, system) = system;
        let integrators = &system.integrators;
        let next_system = match integrators.len() {
            // If no integrators are active, then clone the system.
            0 => system.clone(),
            // If integrators are active, apply them to the system.
            _ => {
                let system = (system_id, system);
                // Apply first integrator.
                let integrator = integrators.get(0).expect("This should never fail.");
                let mut next_system = self.forward_system(integrator, system, neighborhoods);
                // Apply possible other integrators.
                let integrators = integrators.get(1..).expect("This should never fail.");
                for integrator in integrators.iter() {
                    next_system = self.forward_system(integrator, system, neighborhoods);
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
        neighborhoods: &Neighborhoods,
    ) -> UnsizedSystem;

    /// Forward entity to next time-step.
    fn forward_entity(
        &self,
        integrator: &Integrator,
        entity: ((usize, usize), &Box<dyn Entity>),
        neighborhoods: &Neighborhoods,
    ) -> Box<dyn Entity> {
        let (config, state) = (self.get_engine_config(), self.get_current_state());
        let entity = integrator.forward_entity(entity, state, config, neighborhoods);
        entity
    }

    /// Get reference to engine-id.
    fn get_engine_id(&self) -> &usize;

    /// Get reference to state-vector.
    fn get_engine_states(&self) -> &Vec<UnsizedState>;

    /// Get mutable reference to state-vector.
    fn mut_engine_states(&mut self) -> &mut Vec<UnsizedState>;

    /// Get reference to engine-config.
    fn get_engine_config(&self) -> &EngineConfig;

    /// Get mutable reference to engine-config.
    fn mut_engine_config(&mut self) -> &mut EngineConfig;

    /// Add state to state-vector.
    fn add_engine_state(&mut self, state: UnsizedState) {
        self.mut_engine_states().push(state);
    }
    /// Get current state.
    fn get_current_state(&self) -> &UnsizedState {
        let state_id = self.get_engine_states().len() - 1;
        let state = self.get_engine_states().get(state_id).unwrap();
        state
    }
}
