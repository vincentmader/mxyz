use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;

#[get("/simulations/<category>/<sim_id>")]
pub fn route(category: &str, sim_id: &str) -> Template {
    let title = format!("{} (TODO: title)", sim_id);
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
