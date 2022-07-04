#![allow(unused_must_use)]
use mxyz_network::package::Package;
use rocket::fs::{relative, FileServer};
use rocket::{catchers, Catcher, Route};
use rocket_dyn_templates::Template;
use std::sync::mpsc;

/// Rocket Server
pub struct RocketServer {
    routes: Vec<Route>,
    catchers: Vec<Catcher>,
    file_server: FileServer,
}

impl RocketServer {
    /// Creates a new Server instance
    pub fn new() -> Self {
        let routes = crate::http::get_all_routes();
        let catchers = catchers![crate::http::error::not_found::route,];
        let file_server = FileServer::from(relative!("static"));
        RocketServer {
            routes,
            catchers,
            file_server,
        }
    }

    /// Starts the Server aynchronously
    pub async fn init(self) -> Result<(), rocket::Error> {
        let db_conn = mxyz_database::establish_connection();
        // Create MPSC channel for Server-Engine Communication.
        let (tx_1, rx_1) = mpsc::channel::<Package>();
        let (tx_2, rx_2) = mpsc::channel::<Package>();
        // Server-Client Communication: Start TCP-Listener in separate thread.
        std::thread::spawn(move || {
            crate::tcp::start_tcp_listener(tx_1, rx_2).unwrap();
        });
        // Server-Engine Communication: Create Engine-Runner w/ MPSC streaming channel.
        std::thread::spawn(move || {
            crate::engine::start_engine_runner(tx_2, rx_1).unwrap();
        });
        // Launch Rocket.
        rocket::build()
            .mount("/", self.routes)
            .mount("/static", self.file_server)
            .attach(crate::misc::cors::CORS)
            .attach(Template::fairing())
            .register("/", self.catchers)
            .launch()
            .await
            .expect("failed to launch Rocket");

        Ok(()) // TODO
    }
}

// type M = Message; // MPSC message type

// use mxyz_engine::Engine;

// pub async fn start_engine(simulation_variant: &str) {
//     let (tx, rx) = mpsc::channel();
//     let simulation_variant = get_simulation_variant_from_str(simulation_variant);
//     let mut engine = Engine::new(rx, tx);
//     engine.init(&simulation_variant);

//     println!("Starting Engine...");
//     std::thread::spawn(move || engine.run());
// }
