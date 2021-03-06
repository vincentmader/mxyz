use rocket::routes;
// use rocket::response::content;
// use rocket::get;
pub mod error;
pub mod index;
pub mod simulation;

/// Returns Vector of all Rocket Routes
pub fn get_all_routes() -> Vec<rocket::Route> {
    routes![index::route, simulation::route,]
}
