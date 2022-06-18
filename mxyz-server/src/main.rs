mod cors;
mod routes;

#[macro_use]
extern crate rocket;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

// ============================================================================

#[rocket::main]
pub async fn main() -> Result<(), rocket::Error> {
    let routes = routes::get_all_routes();
    let catchers = catchers![routes::error::not_found::route,];
    let file_server = FileServer::from(relative!("static"));

    rocket::build()
        .register("/", catchers)
        .mount("/", routes)
        .mount("/static", file_server)
        .attach(cors::CORS)
        .attach(Template::fairing())
        .launch()
        .await
}
