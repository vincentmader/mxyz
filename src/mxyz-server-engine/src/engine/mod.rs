use mxyz_engine::config::EngineConfig;
use mxyz_engine::engine::Engine;
use mxyz_engine::integrator::Integrator;
use mxyz_engine::state::State;
use mxyz_engine::system::System;
use rayon::prelude::*;

pub struct SimulationEngineV2 {
    pub config: EngineConfig,
    engine_id: usize,
    pub states: Vec<State>,
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
            .enumerate()
            .map(|(id, sys)| self.forward_system((id, sys)))
            .collect();
        let state_id = state.state_id + 1;
        State { state_id, systems }
    }
    fn integrate_system(&self, integrator: &Integrator, system: (usize, &System)) -> System {
        let (sys_id, system) = system;
        let entities = system
            .entities
            .par_iter()
            .enumerate()
            .map(|(ent_id, ent)| self.integrate_entity(integrator, ((sys_id, ent_id), ent)))
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
