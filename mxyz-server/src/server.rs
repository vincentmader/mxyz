use crate::cors;
use crate::routes;
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
        let routes = routes::get_all_routes();
        let catchers = catchers![routes::error::not_found::route,];
        let file_server = FileServer::from(relative!("static"));
        RocketServer {
            routes,
            catchers,
            file_server,
        }
    }
    /// Starts the Server aynchronously
    pub async fn start(self) -> Result<(), rocket::Error> {
        // Start TCP Listener in separate thread.
        // TODO test if client can get/post data
        // TODO `move` needed?
        std::thread::spawn(move || {
            tcp::start_tcp_listener().unwrap();
        });
        // Launch Rocket.
        rocket::build()
            .mount("/", self.routes)
            .mount("/static", self.file_server)
            .attach(cors::CORS)
            .attach(Template::fairing())
            .register("/", self.catchers)
            .launch()
            .await
    }
}
