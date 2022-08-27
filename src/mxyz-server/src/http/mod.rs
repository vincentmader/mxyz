pub mod cors;
pub mod page_routes;
use rocket::routes;

/// Returns Vector of all Rocket Routes
pub fn get_all_routes() -> Vec<rocket::Route> {
    routes![page_routes::index::route, page_routes::simulation::route,]
}
