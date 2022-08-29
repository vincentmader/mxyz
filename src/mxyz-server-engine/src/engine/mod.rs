use mxyz_engine::config::EngineConfig;
use mxyz_engine::engine::Engine;
use mxyz_engine::integrator::Integrator;
use mxyz_engine::state::State;
use mxyz_engine::system::System;
use rayon::prelude::*;

pub struct SimulationEngineV2 {
    pub config: EngineConfig,
    engine_id: usize,
    states: Vec<State>,
}
impl SimulationEngineV2 {
    pub fn new(engine_id: usize) -> Self {
        let config = EngineConfig::new();
        let states = vec![];

        SimulationEngineV2 {
            engine_id,
            config,
            states,
        }
    }
}
impl Engine for SimulationEngineV2 {
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
        let entities = system
            .entities
            .par_iter()
            .map(|x| self.integrate_entity(integrator, x))
            .collect();
        System {
            entities,
            integrators: system.integrators.clone(), // todo: move to config
            variant: system.variant.clone(),         // todo: move to config
            system_id: system.system_id,
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
