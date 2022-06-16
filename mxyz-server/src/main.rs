#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket::response::content;
use rocket_dyn_templates::Template;
mod views;

#[get("/test_json")]
fn test_json() -> content::Json<&'static str> {
    let foo = "{ 'hi': 'world' }"; // TODO get from engine's state-vec (or db?)
    content::Json(foo)
}
#[get("/test_bytes")]
fn test_bytes() -> content::Json<&'static str> {
    let foo = "{}"; // TODO bytes!
    content::Json(foo)
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let routes = routes![
        views::index::route,
        views::simulation::route,
        test_json,
        test_bytes
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
