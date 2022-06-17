use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;

#[get("/simulations/<category>/<sim_id>")]
pub fn route(category: &str, sim_id: &str) -> Template {
    let title = get_title_from_sim_id(sim_id);
    let context: HashMap<&str, &str> = [
        ("category", category),
        ("sim_id", sim_id),
        ("title", &title),
    ]
    .iter()
    .cloned()
    .collect();
    Template::render("simulation/base", &context)
}

pub fn get_title_from_sim_id(sim_id: &str) -> String {
    match sim_id {
        "3body-moon" => "Star-Planet-Moon System",
        _ => sim_id, // _ => todo!("TODO: define title for \"{}\"", sim_id),
    }
    .to_string()
}
