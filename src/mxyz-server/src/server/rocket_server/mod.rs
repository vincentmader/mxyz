#![allow(unused_must_use)]
use mxyz_network::mpsc_msg::MpscMessage;
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
    /// Create a new Server instance.
    pub fn new() -> Self {
        // Define HTTP routes (e.g. html or json requests).
        let routes = crate::http::get_all_routes();

        // Define error catchers (e.g. 404).
        let catchers = catchers![crate::http::page_routes::error::not_found::route,];

        // Define static file server location.
        let file_server = FileServer::from(relative!("static"));

        RocketServer {
            routes,
            catchers,
            file_server,
        }
    }

    /// Start the Server aynchronously.
    pub async fn init(self) -> Result<(), rocket::Error> {
        // Create MPSC channel for Server-Engine Communication.
        let (tx_1, rx_1) = mpsc::channel::<MpscMessage>();

        // Start TCP-Listener in separate thread (server-client API).
        std::thread::spawn(move || {
            crate::tcp::start_tcp_listener(tx_1).unwrap();
        });

        // Start Engine-Runner (server-engine API).
        std::thread::spawn(move || {
            // crate::engine::start_engine_runner(rx_1).unwrap();
            let mut engine_runner = crate::engine_runner_v2::EngineRunnerV2::new(rx_1);
            engine_runner.init();
        });

        // Launch Rocket.
        rocket::build()
            .mount("/", self.routes)
            .mount("/static", self.file_server)
            .attach(crate::http::cors::CORS)
            .attach(Template::fairing())
            .register("/", self.catchers)
            .launch()
            .await
            .expect("failed to launch Rocket");

        Ok(()) // TODO
    }
}
