use diesel::PgConnection;
use mxyz_database::models::state::NewState;
use mxyz_engine::config::EngineConfig;
use mxyz_engine::integrator::Integrator;
use mxyz_engine::state::UnsizedState;
use mxyz_engine::system::unsized_system::UnsizedSystem;
use mxyz_engine::Engine;
use rayon::prelude::*;

pub struct SimulationEngineV2 {
    pub config: EngineConfig,
    engine_id: usize,
    pub states: Vec<UnsizedState>,
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
    fn forward_state(&self, state: &UnsizedState) -> UnsizedState {
        let systems = state
            .systems
            .par_iter()
            .enumerate()
            .map(|(id, sys)| self.forward_or_clone_system((id, sys)))
            .collect();

        let state_id = state.state_id + 1;
        // std::fs::write("../log/log_8.txt", &format!("state-id: {:?}", state_id)).unwrap();
        if state_id % self.engine_config().nr_of_steps_between_exports == 0 {
            self.export_states_to_db(state_id);
        }

        UnsizedState { state_id, systems }
    }
    fn forward_system(
        &self,
        integrator: &Integrator,
        system: (usize, &UnsizedSystem),
    ) -> UnsizedSystem {
        let (sys_id, system) = system;
        let entities = system
            .entities
            .par_iter()
            .enumerate()
            .map(|(ent_id, ent)| self.forward_entity(integrator, ((sys_id, ent_id), ent)))
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
impl SimulationEngineV2 {
    pub fn export_states_to_db(&self, state_id: usize) {
        let conn = mxyz_database::establish_connection();
        // std::fs::write("../log/log_1.txt", "").unwrap();
        let last_export_id = std::cmp::max(
            0,
            (state_id as i32) - (self.engine_config().nr_of_steps_between_exports as i32),
        ) as usize;
        let export_state_ids = last_export_id..self.engine_config().step_id.0;
        // std::fs::write("../log/log_2.txt", &format!("{:?}", export_state_ids)).unwrap();
        // TODO export state ids is empty
        for state_id in export_state_ids {
            let state = NewState {
                engine_id: &(*self.engine_id() as i32),
                state_id: &(state_id as i32),
            };
            mxyz_database::models::state::create_state(&conn, state);
        }
    }
}
