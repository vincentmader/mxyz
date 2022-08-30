use mxyz_engine::config::export_variant::ExportVariant;
use mxyz_engine::config::simulation_variant::SimulationVariant;
use mxyz_engine::engine::Engine;
use mxyz_engine::system::SystemVariant;
use mxyz_network::mpsc_msg;
use mxyz_network::mpsc_msg::MpscMessage;
use std::sync::mpsc;

/// Engine Runner
pub struct EngineRunner {
    rx: mpsc::Receiver<MpscMessage>,
}

impl EngineRunner {
    /// Creates a new Engine-Runner instance
    pub fn new(rx: mpsc::Receiver<MpscMessage>) -> Self {
        EngineRunner { rx }
    }

    /// Initializes MPSC Receiver
    pub fn init(&mut self) {
        println!("Initializing Engine Runner...");
        loop {
            self.receive();
        }
    }

    /// Receives MPSC Messages
    pub fn receive(&mut self) {
        println!("Engine Runner listening...");
        let msg = self.rx.recv().unwrap();
        println!("Engine Runner received msg: {:?}", msg);
        match &msg {
            MpscMessage::Command(cmd) => match cmd {
                mpsc_msg::command::Command::AddEngine(engine_id, client_id, simulation_variant) => {
                    self.add_engine(*engine_id, *client_id, simulation_variant)
                }
            },
        };
        println!("Engine-Runner received MPSC msg: {:#?}", msg);
    }

    /// Adds Engine
    pub fn add_engine(
        &mut self,
        engine_id: usize,
        _client_id: usize, // TODO needed?
        simulation_variant: &SimulationVariant,
    ) {
        let simulation_variant = simulation_variant.clone();

        // Create & initialize new simulation engine.
        let mut engine = mxyz_server_engine::SimulationEngineV2::new(engine_id);
        engine.init(Some(simulation_variant));
        engine.config.step_id.1 = usize::MAX;

        // Run engine in new thread.
        std::thread::spawn(move || {
            for _ in 0..engine.config.step_id.1 {
                // Forward engine to next step-id.
                engine.forward_engine();
                // Every so often, export the engine to the database.
                if engine.config.step_id.0 % engine.config.nr_of_steps_between_exports == 0 {
                    export_engine(&mut engine);
                }
            }
        });
        println!("Engine-Runner added engine {}", engine_id);
    }

    /// Removes Engine
    pub fn remove_engine(&mut self, engine_id: &usize) {
        // TODO stop compute-loop
        // - communicate with engines via MPSC?
        // TODO remove from database
        println!("Engine-Runner removed engine {}", engine_id);
    }
}

/// Exports Engine
pub fn export_engine<T: Engine>(engine: &mut T) {
    // Get state-ids.
    let states_to_save = get_unsaved_state_ids(engine);
    // Choose export method.
    match engine.engine_config().export_variant {
        ExportVariant::ToFile => export_to_file(engine, &states_to_save),
        ExportVariant::ToDatabase => export_to_database(engine, &states_to_save),
    }
    // Update step-id of last export.
    let state_id = Some(engine.engine_config().step_id.0);
    engine.engine_config_mut().last_export_step_id = state_id;
}

/// Exports Engine to Database
pub fn export_to_database<T: Engine>(engine: &mut T, states_to_save: &Vec<usize>) {
    let conn = mxyz_database::establish_connection();

    // Loop over unsaved states.
    for state_id in states_to_save.iter() {
        let state = engine.engine_states().get(*state_id).unwrap();
        // Loops over systems.
        for system in state.systems.iter() {
            // Gets ids for engine, system, & system-variant (via conversion: enum -> usize).
            let engine_id = engine.engine_id();
            let system_id = system.system_id;
            let system_variant_id: usize = system.variant.clone().into(); // TODO remove clone

            // Choose export format. (depends on system variant)
            match &system.variant {
                SystemVariant::EntitiesV1 => {
                    for (ent_id, ent) in system.entities.iter().enumerate() {
                        let db_entity = mxyz_database::models::entity_v1::NewEntityV1 {
                            // ids of engien, state, system & entity
                            engine_id: &(*engine.engine_id() as i32),
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
            engine_id: &(*engine.engine_id() as i32),
            state_id: &(*state_id as i32),
        };
        mxyz_database::models::state::create_state(&conn, db_state);
    }
}

/// Exports Engine to File
pub fn export_to_file<T: Engine>(engine: &mut T, states_to_save: &Vec<usize>) {
    // Get simulation variant & state vector.
    let simulation_variant = engine.engine_config().simulation_variant.as_ref().unwrap();
    let states = engine.engine_states();
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

/// Gets state-ids of states not yet saved to database.
pub fn get_unsaved_state_ids<T: Engine>(engine: &T) -> Vec<usize> {
    engine
        .engine_states()
        .iter()
        .filter(|state| {
            state.state_id
                >= match engine.engine_config().last_export_step_id {
                    // If last-export-id is None, load all states since 0.
                    None => 0,
                    // If not None, load states since last-export-id + 1.
                    Some(e) => e + 1,
                }
        })
        .map(|state| state.state_id)
        .collect()
}
