#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;
mod views;

use rocket::response::content;
#[get("/test")]
fn json() -> content::Json<&'static str> {
    let foo = "{ 'hi': 'world' }"; // TODO get from engine's state-vec (or db?)
    content::Json(foo)
}

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let routes = routes![views::index::route, views::simulation::route, json];
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
