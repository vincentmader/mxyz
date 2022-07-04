use mxyz_engine::config::EngineConfig;
use mxyz_engine::config::ExportVariant;
use mxyz_engine::state::State;
use mxyz_engine::Engine;
use mxyz_network::package::request::Request;
use mxyz_network::package::Package;
use mxyz_universe::entity::attribute::*;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::system::SystemVariant;
use std::sync::mpsc;

pub struct EngineRunner {
    rx: mpsc::Receiver<Package>,
    tx: mpsc::Sender<Package>,
    engines: Vec<Engine>,
}
impl EngineRunner {
    /// Creates a new Engine-Runner instance
    pub fn new(tx: mpsc::Sender<Package>, rx: mpsc::Receiver<Package>) -> Self {
        let engines = vec![];
        EngineRunner { rx, tx, engines }
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
                Request::AddEngine(simulation_variant) => self.add_engine(simulation_variant),
                Request::RemoveEngine(engine_id) => self.remove_engine(engine_id),
                _ => todo!(),
            },
            Package::Response(res) => match res {
                _ => todo!(),
            },
        };
        println!("Server received MPSC msg: {:#?}", msg);
    }

    /// Adds Engine
    pub fn add_engine(&mut self, simulation_variant: &SimulationVariant) {
        let engine_id = 0;

        let simulation_variant = simulation_variant.clone();
        // std::thread::spawn(move || {
        // Connect to database & determine Client- & Engine-ID.
        let db_conn = mxyz_database::establish_connection();
        let engine_id = mxyz_database::models::engine::get_db_engines().len();
        let client_id = mxyz_database::models::client::get_db_clients().len();
        // Create MPSC channel.
        // - TODO change message type
        let (tx, rx) = mpsc::channel::<usize>();
        // Create & initialize new Simulation Engine.
        let mut engine = Engine::new(client_id, engine_id, rx, tx);
        engine.init(&Some(simulation_variant));
        // Save Engine to Database. (actually: only ID)
        let db_engine = mxyz_database::models::engine::NewEngine {
            client_id: &(client_id as i32),
            engine_id: &(engine_id as i32),
        };
        mxyz_database::models::engine::create_engine(&db_conn, db_engine);
        // Run Engine in new thread.
        std::thread::spawn(move || {
            for _ in 0..engine.config.step_id.1 {
                engine.step();
                if engine.config.step_id.0 % engine.config.nr_of_steps_between_exports == 0 {
                    export(&mut engine);
                }
            }
        });
        std::thread::spawn(|| loop {});
        // });
        //...
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
    let states = engine.get_unsaved_state_ids();

    // let config = &mut engine.config;
    // println!("MXYZ-Engine: Exporting...");
    // Choose export method.
    match engine.config.export_variant {
        ExportVariant::ToFile => export_to_file(engine, &states),
        ExportVariant::ToDatabase => export_to_database(engine, &states),
    }
    // Update step-id of last export.
    engine.config.last_export_step_id = Some(engine.config.step_id.0);
    println!(
        "Exported states {} to {}",
        states[0],
        states[states.len() - 1]
    );
}

/// Exports Engine to File
pub fn export_to_file(engine: &mut Engine, states_to_save: &Vec<usize>) {
    let simulation_variant = 0; // TODO
    let out_dir = format!("./mxyz-engine/output/{}", simulation_variant);
    // Loop over unsaved States.
    for state_id in states_to_save.iter() {
        let state = engine.states.get(*state_id).unwrap();
        // Save to File.
        let path = format!("{}/{}.txt", out_dir, state_id);
        std::fs::write(path, format!("{:#?}", state)).unwrap();
    }
}

// TODO
/// Exports Engine to Database
pub fn export_to_database(engine: &mut Engine, states_to_save: &Vec<usize>) {
    // Establish Connection.
    let conn = mxyz_database::establish_connection();

    // Loop over unsaved States.
    for state_id in states_to_save.iter() {
        // println!("{}", state_id);
        let state = engine.states.get(*state_id).unwrap();
        // Loops over Systems.
        for system in state.systems.iter() {
            //         // let _system_variant_id = System::get_variant_id(&system.variant);
            let system_id = system.system_id;
            match &system.variant {
                SystemVariant::Planets(system) => {
                    // Loop over Entities.
                    for (planet_id, planet) in system.entities.iter().enumerate() {
                        let db_planet = mxyz_database::models::planet::NewPlanet {
                            client_id: &(engine.client_id as i32),
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
            //         let entity_variant_id = match &system.variant {
            //             SystemVariant::Planets(_) => 0,
            //             SystemVariant::PhysicalObjects(_) => 1,
            //             _ => todo!(),
            //         };
            //         let db_system = mxyz_database::models::NewSystem {
            //             state_id: &(state_id as i32),
            //             system_id: &(system_id as i32),
            //             entity_variant_id: &(entity_variant_id as i32),
            //         };
            //         mxyz_database::create_system(&conn, db_system);
        }
        let db_state = mxyz_database::models::state::NewState {
            client_id: &(engine.client_id as i32),
            engine_id: &(engine.engine_id as i32),
            state_id: &(*state_id as i32),
        };
        mxyz_database::create_state(&conn, db_state);
    }
}
// ============================================================================
// TODO move to separate module (?)

// impl Engine {

//     pub fn get_state_ids() -> Vec<usize> {
//         use diesel::prelude::*;
//         use mxyz_database::models::State;
//         // use mxyz_database::schema::systems;
//         use mxyz_database::schema::states::dsl::*;
//         /// Establishes Connection.
//         let connection = mxyz_database::establish_connection();
//         let results = states
//             .load::<State>(&connection)
//             .expect("Error loading states");
//         results.iter().map(|i| i.state_id as usize).collect()
//     }

//     pub fn get_system_ids(state_id_query: usize) -> Vec<usize> {
//         use diesel::prelude::*;
//         use mxyz_database::models::System;
//         use mxyz_database::schema::systems::dsl::*;
//         /// Establishes Connection.
//         let connection = mxyz_database::establish_connection();
//         let results = systems
//             .filter(state_id.eq(state_id_query as i32))
//             .load::<System>(&connection)
//             .expect("Error loading systems");
//         results.iter().map(|i| i.system_id as usize).collect()
//     }

//     pub fn get_entities(state_id_query: usize, system_id_query: usize) -> Vec<Planet> {
//         use diesel::prelude::*;
//         use mxyz_database::models::Planet;
//         use mxyz_database::schema::planets::dsl::*;
//         /// Establishes Connection.
//         let connection = mxyz_database::establish_connection();

//         let results = planets
//             .filter(state_id.eq(state_id_query as i32))
//             .filter(system_id.eq(system_id_query as i32))
//             .load::<Planet>(&connection)
//             .expect("Error loading planets");

//         println!("planets:\n{:?}", results);
//         // planets
//         vec![]
//     }

// }
