#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

// use rocket::response::stream::Event;
// use rocket::tokio::time::Duration;

// use mxyz_server::events;
// use mxyz_server::views;
// use mxyz_server;
mod views;
// use mxyz_server::run_server;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let routes = routes![
        views::index::route,
        // views::simulation::route,
        // views::chronos::route,
    ];
    let file_server = FileServer::from(relative!("static"));

    rocket::build()
        .mount("/", routes)
        .mount("/static", file_server)
        .attach(Template::fairing())
        .launch()
        .await
}
