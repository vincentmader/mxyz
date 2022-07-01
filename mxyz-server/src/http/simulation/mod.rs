extern crate rocket_dyn_templates;
use mxyz_engine::state::preset::SimulationVariant;
use mxyz_engine::Engine;
use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::sync::mpsc;
pub mod utils;

#[get("/simulations/<category>/<simulation_variant>")]
pub async fn route(category: &str, simulation_variant: &str) -> Template {
    // Server-Engine Communication: Create Engine-Runner w/ MPSC streaming channel.
    std::thread::spawn(move || {
        let (tx, rx) = mpsc::channel();
        let mut engine_runner = EngineRunner::new(rx, tx);
        engine_runner.init();
    });

    // start_engine(simulation_variant).await;
    // TODO
    // - get nr of engines from database
    // - client_id = nr_of_engines
    let client_id = 0;
    let client_id = format!("{}", client_id); // TODO make context serializable struct

    let title = utils::get_title_from_simulation_variant(category, simulation_variant);
    let context: HashMap<&str, &str> = [
        ("category", category),
        ("simulation_variant", simulation_variant),
        ("title", &title),
        ("client_id", &client_id),
    ]
    .iter()
    .cloned()
    .collect();
    Template::render("simulation/base", &context)
}

#[derive(Debug)]
pub enum Message {
    AddEngine,
    RemoveEngine(usize),
}

type M = Message; // MPSC message type

pub struct EngineRunner {
    rx: mpsc::Receiver<M>,
    tx: mpsc::Sender<M>,
    engines: Vec<Engine>,
}
impl EngineRunner {
    pub fn new(rx: mpsc::Receiver<M>, tx: mpsc::Sender<M>) -> Self {
        let engines = vec![];
        EngineRunner { rx, tx, engines }
    }
    pub fn init(&mut self) {
        println!("Initializing Engine Runner...");
    }
    pub fn send(&self, msg: M) {
        println!("Server sending MPSC msg: {:#?}", msg);
        self.tx.send(msg).unwrap();
    }
    pub fn receive(&self) {
        let msg = self.rx.recv().unwrap();
        println!("Server received MPSC msg: {:#?}", msg);
    }
    pub fn add_engine(&mut self, simulation_variant: SimulationVariant) {
        // let engine = Engine::new();
    }
    pub fn remove_engine(&mut self, engine_id: usize) {}
}

// ============================================================================

// use mxyz_engine::Engine;

// pub async fn start_engine(simulation_variant: &str) {
//     let (tx, rx) = mpsc::channel();
//     let simulation_variant = get_simulation_variant_from_str(simulation_variant);
//     let mut engine = Engine::new(rx, tx);
//     engine.init(&simulation_variant);

//     println!("Starting Engine...");
//     std::thread::spawn(move || engine.run());
// }
