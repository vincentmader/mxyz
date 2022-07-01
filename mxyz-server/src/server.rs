use crate::http;
use crate::misc;
use crate::tcp;
use mxyz_engine::Engine;
use rocket::fs::{relative, FileServer};
use rocket::{catchers, Catcher, Route};
use rocket_dyn_templates::Template;
use std::sync::mpsc;
use std::thread;

type M = usize; // MPSC message type

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
    pub async fn start(self) -> Result<(), rocket::Error> {
        // Create a simple streaming channel
        let (tx, rx) = mpsc::channel();
        let mut engine_runner: EngineRunner = EngineRunner::new(rx, tx);
        engine_runner.init();
        // let engine_runner = EngineRunner::new(rx);

        // Starts TCP Listener in separate thread.
        std::thread::spawn(move || {
            tcp::start_tcp_listener().unwrap();
        });
        // Launches Rocket.
        rocket::build()
            .mount("/", self.routes)
            .mount("/static", self.file_server)
            .attach(misc::cors::CORS)
            .attach(Template::fairing())
            .register("/", self.catchers)
            .launch()
            .await
    }
}

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
    pub fn init(&mut self) {}
    pub fn send(&self, msg: M) {
        // thread::spawn(move || {
        self.tx.send(msg).unwrap();
        // });
    }
    pub fn receive(&self) {}
    pub fn add_engine(&mut self) {}
    pub fn remove_engine(&mut self, engine_id: usize) {}
}

// pub struct EngineRunner<T> {
//     rx: mpsc::Receiver<T>,
// }
// impl<T> EngineRunner<T> {
//     pub fn new(rx: mpsc::Receiver<T>) -> Self {
//         EngineRunner { rx }
//     }
// }
