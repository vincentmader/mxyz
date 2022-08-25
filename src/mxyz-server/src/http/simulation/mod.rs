extern crate rocket_dyn_templates;
use rocket::get;
use rocket_dyn_templates::Template;
use std::collections::HashMap;
pub mod utils;

#[get("/simulations/<category>/<simulation_variant>")]
pub async fn route(category: &str, simulation_variant: &str) -> Template {
    // start_engine(simulation_variant).await;
    // TODO
    // - get nr of engines from database
    // - client_id = nr_of_engines
    // let client_id = mxyz_database::models::client::get_db_clients().len();
    // let client_id = format!("{}", client_id); // TODO make context serializable struct

    let title = utils::get_title_from_simulation_variant(category, simulation_variant);
    let context: HashMap<&str, &str> = [
        ("category", category),
        ("simulation_variant", simulation_variant),
        ("title", &title),
        // ("client_id", &client_id),
    ]
    .iter()
    .cloned()
    .collect();
    Template::render("simulation/base", &context)
}
