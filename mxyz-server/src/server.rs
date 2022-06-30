use crate::http;
use crate::misc;
use crate::tcp;
use rocket::fs::{relative, FileServer};
use rocket::{catchers, Catcher, Route};
use rocket_dyn_templates::Template;

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
