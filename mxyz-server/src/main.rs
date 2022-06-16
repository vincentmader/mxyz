#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::content;
use rocket_dyn_templates::Template;
mod views;

// WORKS! =====================================================================
#[get("/test_json")]
fn test_json() -> content::Json<&'static str> {
    let foo = "{ 'hi': 'world' }"; // TODO get from engine's state-vec (or db?)
    content::Json(foo)
}
// WORKS! =====================================================================
#[get("/test_download")]
fn test_download() -> Vec<u8> {
    let bytes = vec![0, 1, 2];
    bytes
}
// ==== ? =====================================================================
#[get("/test_bytes")]
// fn test_bytes() -> content::Json<&'static str> {
fn test_bytes() -> Vec<u8> {
    let bytes = vec![0, 1, 2];
    bytes
}
// ==== ? =====================================================================
#[get("/test_tcp")]
fn test_tcp() -> content::Json<&'static str> {
    tcp().unwrap();
    let foo = "{}"; // TODO bytes!
    content::Json(foo)
}
use std::net::{TcpListener, TcpStream};
fn handle_client(stream: TcpStream) {
    // ...
}
fn tcp() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
// ============================================================================

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let routes = routes![
        views::index::route,
        views::simulation::route,
        test_json,
        test_bytes,
        test_tcp,
        test_download,
    ];
    let catchers = catchers![views::error::not_found::route,];
    let file_server = FileServer::from(relative!("static"));

    rocket::build()
        .register("/", catchers)
        .mount("/", routes)
        .mount("/static", file_server)
        .attach(Template::fairing())
        .launch()
        .await
}
