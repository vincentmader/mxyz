use mxyz_engine::config::EngineConfig;
use mxyz_engine::integrator::Integrator;
use mxyz_engine::state::UnsizedState;
use mxyz_engine::system::unsized_system::UnsizedSystem;
use mxyz_engine::Engine;

pub struct SimulationEngineV1 {
    pub config: EngineConfig,
    engine_id: usize,
    pub states: Vec<UnsizedState>,
}
impl SimulationEngineV1 {
    pub fn new(engine_id: usize) -> Self {
        let config = EngineConfig::new();
        let states = vec![];

        SimulationEngineV1 {
            engine_id,
            config,
            states,
        }
    }
}
impl Engine for SimulationEngineV1 {
    fn forward_state(&self, state: &UnsizedState) -> UnsizedState {
        let systems = state
            .systems
            .iter()
            .enumerate()
            .map(|(sys_id, sys)| self.forward_system((sys_id, sys)))
            .collect();
        let state_id = state.state_id + 1;
        UnsizedState { state_id, systems }
    }
    fn integrate_system(
        &self,
        integrator: &Integrator,
        system: (usize, &UnsizedSystem),
    ) -> UnsizedSystem {
        let (system_id, system) = system;
        let entities = system
            .entities
            .iter()
            .enumerate()
            .map(|(ent_id, x)| self.integrate_entity(integrator, ((system_id, ent_id), x)))
            .collect();
        UnsizedSystem {
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
    fn engine_states(&self) -> &Vec<UnsizedState> {
        &self.states
    }
    fn engine_states_mut(&mut self) -> &mut Vec<UnsizedState> {
        &mut self.states
    }
}
