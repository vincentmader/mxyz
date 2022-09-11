use mxyz_engine::config::export_variant::ExportVariant;
use mxyz_engine::config::EngineConfig;
use mxyz_engine::integrator::Integrator;
use mxyz_engine::neighborhoods::Neighborhoods;
use mxyz_engine::state::UnsizedState;
use mxyz_engine::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
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
    fn run(&mut self) {
        for _ in 0..self.config.step_id.1 {
            // Forward engine to next step-id.
            self.forward_engine();
            // Every so often, export the engine to the database.
            if self.config.step_id.0 % self.config.nr_of_steps_between_exports == 0 {
                self.export_engine();
            }
        }
    }
    fn forward_state(&self, state: &UnsizedState, neighborhoods: &Neighborhoods) -> UnsizedState {
        let systems = state
            .systems
            .par_iter()
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
            .par_iter()
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

impl SimulationEngineV2 {
    /// Export Engine.
    pub fn export_engine(&mut self) {
        // Get state-ids.
        let states_to_save = get_unsaved_state_ids(self);
        // Choose export method.
        match self.config.export_variant {
            ExportVariant::ToFile => export_to_file(self, &states_to_save),
            ExportVariant::ToDatabase => export_to_database(self, &states_to_save), // TODO
        }
        // Update step-id of last export.
        let state_id = Some(self.config.step_id.0);
        self.config.last_export = state_id;
    }
}

pub fn export_to_database<T: Engine>(engine: &mut T, states_to_save: &Vec<usize>) {
    let conn = mxyz_database::establish_connection();

    // Loop over unsaved states.
    for state_id in states_to_save.iter() {
        let state = engine.get_engine_states().get(*state_id).unwrap();
        // Loops over systems.
        for system in state.systems.iter() {
            // Gets ids for engine, system, & system-variant (via conversion: enum -> usize).
            let engine_id = engine.get_engine_id();
            let system_id = system.system_id;
            let system_variant_id: usize = system.variant.clone().into(); // TODO remove clone

            // Choose export format. (depends on system variant)
            match &system.variant {
                UnsizedSystemVariant::EntitiesV1 => {
                    for (ent_id, ent) in system.entities.iter().enumerate() {
                        let db_entity = mxyz_database::models::entity_v1::NewEntityV1 {
                            // ids of engien, state, system & entity
                            engine_id: &(*engine.get_engine_id() as i32),
                            state_id: &(*state_id as i32),
                            system_id: &(system_id as i32),
                            entity_id: &(ent_id as i32),
                            // mass
                            mass: &ent.get_mass(),
                            // position
                            pos_x: &ent.get_position()[0],
                            pos_y: &ent.get_position()[1],
                            pos_z: &ent.get_position()[2],
                            // velocity
                            vel_x: &ent.get_velocity()[0],
                            vel_y: &ent.get_velocity()[1],
                            vel_z: &ent.get_velocity()[2],
                            // charge
                            charge: &ent.get_charge(),
                        };
                        mxyz_database::models::entity_v1::create_entity_v1(&conn, db_entity);
                    }
                }
                _ => todo!(),
            }
            // Save system to database.
            let db_system = mxyz_database::models::system::NewSystem {
                engine_id: &(*engine_id as i32),
                state_id: &(*state_id as i32),
                system_id: &(system_id as i32),
                system_variant_id: &(system_variant_id as i32),
            };
            mxyz_database::models::system::create_system(&conn, db_system);
        }
        // Save state to database.
        let db_state = mxyz_database::models::state::NewState {
            engine_id: &(*engine.get_engine_id() as i32),
            state_id: &(*state_id as i32),
        };
        mxyz_database::models::state::create_state(&conn, db_state);
    }
}

/// Export Engine to File.
pub fn export_to_file<T: Engine>(engine: &mut T, states_to_save: &Vec<usize>) {
    // Get simulation variant & state vector.
    let simulation_variant = engine
        .get_engine_config()
        .simulation_variant
        .as_ref()
        .unwrap();
    let states = engine.get_engine_states();
    // Convert simulation variant to integer representation in out-path.
    let simulation_variant: usize = simulation_variant.clone().into();
    let out_dir = format!("./mxyz-engine/output/{:?}", simulation_variant);
    // Loop over unsaved states.
    for state_id in states_to_save.iter() {
        let state = states.get(*state_id).unwrap();
        // Save to file.
        let path = format!("{}/{}.txt", out_dir, state_id);
        std::fs::write(path, format!("{:#?}", state)).unwrap();
    }
}

/// Get state-ids of states not yet saved to database.
pub fn get_unsaved_state_ids<T: Engine>(engine: &T) -> Vec<usize> {
    engine
        .get_engine_states()
        .iter()
        .filter(|state| {
            state.state_id
                >= match engine.get_engine_config().last_export {
                    // If last-export is None, load all states since 0.
                    None => 0,
                    // If not None, load states since last-export-id + 1.
                    Some(e) => e + 1,
                }
        })
        .map(|state| state.state_id)
        .collect()
}
