use mxyz_engine::config::EngineConfig;
use mxyz_engine::engine::Engine_;
use mxyz_engine_universe::integrator::Integrator;
use mxyz_engine_universe::state::State;
use mxyz_engine_universe::system::System;
use rayon::prelude::*;

pub struct SimulationEngineV2Server {
    pub config: EngineConfig,
    engine_id: usize,
    states: Vec<State>,
}
impl SimulationEngineV2Server {
    pub fn new(engine_id: usize) -> Self {
        // Create new engine-config.
        let config = EngineConfig::new();
        // Initialize empty state-vector.
        let states = vec![];

        SimulationEngineV2Server {
            engine_id,
            config,
            states,
        }
    }
}
impl Engine_ for SimulationEngineV2Server {
    fn forward_state(&self, state: &State) -> State {
        // Forward systems.
        let systems = state
            .systems
            .par_iter()
            .map(|sys| self.forward_system(sys))
            .collect();
        // Update state-id.
        let state_id = state.state_id + 1;
        // Return next state.
        State { state_id, systems }
    }
    fn integrate_system(&self, integrator: &Integrator, system: &System) -> System {
        todo!();
    }
    fn engine_config(&self) -> &EngineConfig {
        &self.config
    }
    fn engine_config_mut(&mut self) -> &mut EngineConfig {
        &mut self.config
    }
    fn engine_id(&self) -> &usize {
        &self.engine_id
    }
    fn engine_states(&self) -> &Vec<State> {
        &self.states
    }
    fn engine_states_mut(&mut self) -> &mut Vec<State> {
        &mut self.states
    }
}
