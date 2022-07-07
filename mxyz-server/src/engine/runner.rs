use mxyz_engine::config::ExportVariant;
use mxyz_engine::Engine;
use mxyz_network::package::command::Command;
use mxyz_network::package::request::Request;
use mxyz_network::package::Package;
use mxyz_universe::entity::attribute::*;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::system::SystemVariant;
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
                Request::AddEngine(client_id, simulation_variant) => {
                    self.add_engine(*client_id, simulation_variant)
                }
                Request::RemoveEngine(engine_id) => self.remove_engine(engine_id),
                _ => todo!(),
            },
            Package::Command(cmd) => match cmd {
                Command::SaveStatesToDatabase => {}
            },
            _ => todo!(),
        };
        println!("Engine-Runner received MPSC msg: {:#?}", msg);
    }

    /// Adds Engine
    pub fn add_engine(&mut self, client_id: usize, simulation_variant: &SimulationVariant) {
        let simulation_variant = simulation_variant.clone();

        // Save Engine to Database. (actually: only ID)
        let db_conn = mxyz_database::establish_connection();
        let db_engine = mxyz_database::models::engine::create_engine(&db_conn, client_id);
        let engine_id = db_engine.engine_id as usize;

        // Create & initialize new Simulation Engine.
        let mut engine = Engine::new(engine_id);
        engine.init(Some(simulation_variant));

        // Run Engine in new thread.
        std::thread::spawn(move || {
            // for _ in 0..1000 {
            for _ in 0..engine.config.step_id.1 {
                engine.step();
                if engine.config.step_id.0 % engine.config.nr_of_steps_between_exports == 0 {
                    export(&mut engine);
                }
            }
        });
        println!("Engine-Runner added engine {}", engine_id);
    }

    /// Removes Engine
    pub fn remove_engine(&mut self, engine_id: &usize) {
        // ...
        println!("Engine-Runner removed engine {}", engine_id);
    }
}

/// Exports Engine
pub fn export(engine: &mut Engine) {
    let states_to_save = engine.get_unsaved_state_ids();

    // Choose export method.
    match engine.config.export_variant {
        ExportVariant::ToFile => export_to_file(engine, &states_to_save),
        ExportVariant::ToDatabase => export_to_database(engine, &states_to_save),
    }

    // Update step-id of last export.
    engine.config.last_export_step_id = Some(engine.config.step_id.0);

    // println!(
    //     "Exported states {} to {}",
    //     states_to_save[0],
    //     states_to_save[states_to_save.len() - 1]
    // );
}

/// Exports Engine to File
pub fn export_to_file(engine: &mut Engine, states_to_save: &Vec<usize>) {
    // Get simulation variant.
    let simulation_variant = engine.config.simulation_variant.as_ref().unwrap();
    // Convert simulation variant to integer representation in out-path.
    let simulation_variant: usize = simulation_variant.clone().into();
    let out_dir = format!("./mxyz-engine/output/{:?}", simulation_variant);

    // Loop over unsaved States.
    for state_id in states_to_save.iter() {
        let state = engine.states.get(*state_id).unwrap();
        // Save to File.
        let path = format!("{}/{}.txt", out_dir, state_id);
        std::fs::write(path, format!("{:#?}", state)).unwrap();
    }
}

/// Exports Engine to Database
pub fn export_to_database(engine: &mut Engine, states_to_save: &Vec<usize>) {
    let conn = mxyz_database::establish_connection();

    // Loop over unsaved States.
    for state_id in states_to_save.iter() {
        let state = engine.states.get(*state_id).unwrap();
        // Loops over Systems.
        for system in state.systems.iter() {
            let engine_id = 1; // TODO
            let system_id = system.system_id;
            let system_variant_id: usize = system_id.into();

            // Export format depends on System Variant.
            match &system.variant {
                SystemVariant::Planets(system) => {
                    // Loop over Entities.
                    for (planet_id, planet) in system.entities.iter().enumerate() {
                        let db_planet = mxyz_database::models::planet::NewPlanet {
                            engine_id: &(engine.engine_id as i32),
                            state_id: &(*state_id as i32),
                            planet_id: &(planet_id as i32),
                            system_id: &(system_id as i32),
                            mass: &planet.get_mass(),
                            pos_x: &planet.get_position()[0],
                            pos_y: &planet.get_position()[1],
                            pos_z: &planet.get_position()[2],
                            vel_x: &planet.get_velocity()[0],
                            vel_y: &planet.get_velocity()[1],
                            vel_z: &planet.get_velocity()[2],
                        };
                        mxyz_database::create_planet(&conn, db_planet);
                    }
                }
                _ => {}
            }
            // Save system to database.
            let db_system = mxyz_database::models::system::NewSystem {
                engine_id: &(engine_id as i32),
                state_id: &(*state_id as i32),
                system_id: &(system_id as i32),
                system_variant_id: &(system_variant_id as i32),
            };
            mxyz_database::create_system(&conn, db_system);
        }
        // Save state to database.
        let db_state = mxyz_database::models::state::NewState {
            engine_id: &(engine.engine_id as i32),
            state_id: &(*state_id as i32),
        };
        mxyz_database::create_state(&conn, db_state);
    }
}
