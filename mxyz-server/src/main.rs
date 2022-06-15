#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
mod views;

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let routes = routes![views::index::route, views::simulation::route];
    let catchers = catchers![views::error::not_found::route,];
    let file_server = FileServer::from(relative!("static"));

    rocket::build()
        .mount("/", routes)
        .mount("/static", file_server)
        .attach(Template::fairing())
        .register("/", catchers)
        .launch()
        .await
}
