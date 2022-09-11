use mxyz_engine::config::EngineConfig;
use mxyz_engine::integrator::Integrator;
use mxyz_engine::neighborhoods::Neighborhoods;
use mxyz_engine::state::UnsizedState;
use mxyz_engine::system::unsized_system::UnsizedSystem;
use mxyz_engine::Engine;
//

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
    fn run(&mut self) {
        todo!()
    }
    fn forward_state(&self, state: &UnsizedState, neighborhoods: &Neighborhoods) -> UnsizedState {
        let systems = state
            .systems
            .iter()
            .enumerate()
            .map(|(sys_id, sys)| self.forward_or_clone_system((sys_id, sys), neighborhoods))
            .collect();
        let state_id = state.state_id + 1;
        UnsizedState { state_id, systems }
    }

    fn forward_system(
        &self,
        integrator: &Integrator,
        system: (usize, &UnsizedSystem),
        neighborhoods: &Neighborhoods,
    ) -> UnsizedSystem {
        let (system_id, system) = system;
        let entities = system
            .entities
            .iter()
            .enumerate()
            .map(|(ent_id, ent)| {
                self.forward_entity(integrator, ((system_id, ent_id), ent), neighborhoods)
            })
            .collect();
        UnsizedSystem {
            entities,
            integrators: system.integrators.clone(), // todo: move to config
            variant: system.variant.clone(),         // todo: move to config
            system_id: system.system_id,
        }
    }

    fn get_engine_config(&self) -> &EngineConfig {
        &self.config
    }
    fn mut_engine_config(&mut self) -> &mut EngineConfig {
        &mut self.config
    }
    fn get_engine_id(&self) -> &usize {
        &self.engine_id
    }
    fn get_engine_states(&self) -> &Vec<UnsizedState> {
        &self.states
    }
    fn mut_engine_states(&mut self) -> &mut Vec<UnsizedState> {
        &mut self.states
    }
}
