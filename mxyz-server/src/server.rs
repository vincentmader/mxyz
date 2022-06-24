use crate::cors;
use crate::routes;
use rocket::fs::{relative, FileServer};
use rocket::{catchers, Catcher, Route};
use rocket_dyn_templates::Template;

/// Rocket Server
pub struct RocketServer {
    routes: Vec<Route>,
    file_server: FileServer,
    catchers: Vec<Catcher>,
}
impl RocketServer {
    /// Creates a new Server instance
    pub fn new() -> Self {
        let routes = routes::get_all_routes();
        let file_server = FileServer::from(relative!("static"));
        let catchers = catchers![routes::error::not_found::route,];
        RocketServer {
            routes,
            file_server,
            catchers,
        }
    }
    /// Starts the Server aynchronously
    pub async fn start(self) -> Result<(), rocket::Error> {
        // Start TCP Listener in separate thread.
        std::thread::spawn(move || {
            start_tcp_listener().unwrap();
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

use std::io;
use tokio::net::{TcpListener, TcpStream};

const HOST: &str = "127.0.0.1";
const PORT: usize = 8080;

// ============================================================================

// socket = stream
async fn process_socket<T>(socket: T) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    // let mut connection = Connection::new(socket);

    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);

    //     // Respond with an error
    //     let response = Frame::Error("unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }
}

#[tokio::main]
async fn start_tcp_listener() -> io::Result<()> {
    let address = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(&address).await.unwrap();
    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
