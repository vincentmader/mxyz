use mxyz_config::ExportVariant;
use mxyz_engine::Engine;
use mxyz_network::package::command::Command;
use mxyz_network::package::request::Request;
use mxyz_network::package::Package;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::system::system::SystemVariant;
use std::sync::mpsc;

pub struct EngineRunner {
    rx: mpsc::Receiver<Package>,
}
impl EngineRunner {
    /// Creates a new Engine-Runner instance
    pub fn new(rx: mpsc::Receiver<Package>) -> Self {
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
            Package::Request(req) => match req {
                Request::RemoveEngine(engine_id) => self.remove_engine(engine_id),
                _ => todo!(),
            },
            Package::Command(cmd) => match cmd {
                Command::AddEngine(engine_id, client_id, simulation_variant) => {
                    self.add_engine(*engine_id, *client_id, simulation_variant)
                }
                _ => todo!(),
            },
            _ => todo!(),
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

        // Save engine to database. (actually: only engine-id)
        // let db_conn = mxyz_database::establish_connection();
        // let db_engine = mxyz_database::models::engine::create_engine(&db_conn, client_id);
        // let engine_id = db_engine.engine_id as usize;

        // Create & initialize new simulation engine.
        let mut engine = Engine::new(engine_id);
        engine.init(Some(simulation_variant));
        engine.config.step_id.1 = usize::MAX;

        // Run engine in new thread.
        std::thread::spawn(move || {
            for _ in 0..engine.config.step_id.1 {
                // Forward engine to next step-id.
                engine.forward();
                // Every so often, export the engine to the database.
                if engine.config.step_id.0 % engine.config.nr_of_steps_between_exports == 0 {
                    export(&mut engine);
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
pub fn export(engine: &mut Engine) {
    // Get state-ids.
    let states_to_save = engine.get_unsaved_state_ids();
    // Choose export method.
    match engine.config.export_variant {
        ExportVariant::ToFile => export_to_file(engine, &states_to_save),
        ExportVariant::ToDatabase => export_to_database(engine, &states_to_save),
    }
    // Update step-id of last expor.
    engine.config.last_export_step_id = Some(engine.config.step_id.0);
}

/// Exports Engine to Database
pub fn export_to_database(engine: &mut Engine, states_to_save: &Vec<usize>) {
    let conn = mxyz_database::establish_connection();

    // Loop over unsaved states.
    for state_id in states_to_save.iter() {
        let state = engine.states.get(*state_id).unwrap();
        // Loops over systems.
        for system in state.systems.iter() {
            // Gets ids for engine, system, & system-variant (via conversion: enum -> usize).
            let engine_id = engine.engine_id;
            let system_id = system.system_id;
            let system_variant_id: usize = system.variant.clone().into(); // TODO remove clone

            // Choose export format. (depends on system variant)
            match &system.variant {
                SystemVariant::EntitiesV1 => {
                    for (ent_id, ent) in system.entities.iter().enumerate() {
                        let db_entity = mxyz_database::models::entity_v1::NewEntityV1 {
                            // ids of engien, state, system & entity
                            engine_id: &(engine.engine_id as i32),
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
                engine_id: &(engine_id as i32),
                state_id: &(*state_id as i32),
                system_id: &(system_id as i32),
                system_variant_id: &(system_variant_id as i32),
            };
            mxyz_database::models::system::create_system(&conn, db_system);
        }
        // Save state to database.
        let db_state = mxyz_database::models::state::NewState {
            engine_id: &(engine.engine_id as i32),
            state_id: &(*state_id as i32),
        };
        mxyz_database::models::state::create_state(&conn, db_state);
    }
}

/// Exports Engine to File
pub fn export_to_file(engine: &mut Engine, states_to_save: &Vec<usize>) {
    // Get simulation variant.
    let simulation_variant = engine.config.simulation_variant.as_ref().unwrap();
    // Convert simulation variant to integer representation in out-path.
    let simulation_variant: usize = simulation_variant.clone().into();
    let out_dir = format!("./mxyz-engine/output/{:?}", simulation_variant);
    // Loop over unsaved states.
    for state_id in states_to_save.iter() {
        let state = engine.states.get(*state_id).unwrap();
        // Save to file.
        let path = format!("{}/{}.txt", out_dir, state_id);
        std::fs::write(path, format!("{:#?}", state)).unwrap();
    }
}
