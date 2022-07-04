use crate::engine;
use crate::http;
use crate::misc;
use mxyz_network::message::Message;
use mxyz_network::mpsc::MpscReceiver;
use mxyz_network::mpsc::MpscSender;
use mxyz_network::tcp;
use rocket::fs::{relative, FileServer};
use rocket::{catchers, Catcher, Route};
use rocket_dyn_templates::Template;
use std::sync::mpsc;

// ============================================================================

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

// TCP address TODO rename/move
const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 1234;

/// Rocket Server
pub struct RocketServer {
    routes: Vec<Route>,
    catchers: Vec<Catcher>,
    file_server: FileServer,
}

impl RocketServer {
    /// Creates a new Server instance
    pub fn new() -> Self {
        let routes = http::get_all_routes();
        let catchers = catchers![http::error::not_found::route,];
        let file_server = FileServer::from(relative!("static"));
        RocketServer {
            routes,
            catchers,
            file_server,
        }
    }

    /// Starts the Server aynchronously
    pub async fn init(self) -> Result<(), rocket::Error> {
        // Create MPSC channel for Server-Engine Communication
        let (tx, rx) = mpsc::channel::<Message>();
        let (mpsc_sender, mpsc_receiver) = mxyz_network::mpsc::create_channel();

        // Server-Client Communication: Start TCP-Listener in separate thread.
        std::thread::spawn(move || {
            tcp::start_tcp_listener(mpsc_sender).unwrap();
        });

        // Start Engine-Runner in separate thread.
        std::thread::spawn(move || {
            engine::start_engine_runner(mpsc_receiver).unwrap();
        });

        // Launch Rocket.
        rocket::build()
            .mount("/", self.routes)
            .mount("/static", self.file_server)
            .attach(misc::cors::CORS)
            .attach(Template::fairing())
            .register("/", self.catchers)
            .launch()
            .await
            .expect("failed to launch Rocket");

        Ok(()) // TODO
    }
}
