use mxyz_engine::config::EngineConfig;
use mxyz_engine::engine::Engine;
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
        let config = EngineConfig::new();
        let states = vec![];

        SimulationEngineV2Server {
            engine_id,
            config,
            states,
        }
    }
}
impl Engine for SimulationEngineV2Server {
    fn forward_state(&self, state: &State) -> State {
        let systems = state
            .systems
            .par_iter()
            .map(|sys| self.forward_system(sys))
            .collect();
        let state_id = state.state_id + 1;
        State { state_id, systems }
    }
    fn integrate_system(&self, integrator: &Integrator, system: &System) -> System {
        let system_id = system.system_id;
        let entities = system
            .entities
            .par_iter()
            .map(|x| self.integrate_entity(integrator, x))
            .collect();
        let integrators = system.integrators.clone();
        let variant = system.variant.clone();
        System {
            system_id,
            entities,
            integrators,
            variant,
        }
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
