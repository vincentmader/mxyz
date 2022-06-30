use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket::get;
use rocket_dyn_templates::Template;

#[get("/simulations/<category>/<sim_id>")]
pub async fn route(category: &str, sim_id: &str) -> Template {
    start_engine(sim_id).await;
    // TODO
    // - get nr of engines from database
    // - client_id = nr_of_engines
    let client_id = 0;
    let client_id = format!("{}", client_id); // TODO make context serializable struct

    let title = get_title_from_sim_id(category, sim_id);
    let context: HashMap<&str, &str> = [
        ("category", category),
        ("sim_id", sim_id),
        ("title", &title),
        ("client_id", &client_id),
    ]
    .iter()
    .cloned()
    .collect();
    Template::render("simulation/base", &context)
}

// ============================================================================

use mxyz_engine::Engine;

pub async fn start_engine(sim_id: &str) {
    let sim_id = get_sim_id_from_str(sim_id);
    let mut engine = Engine::new();
    engine.init(&sim_id);

    println!("Starting Engine...");
    std::thread::spawn(move || engine.run());
}

// ============================================================================

use mxyz_engine::state::preset::SimulationId;
pub fn get_sim_id_from_str(sim_id: &str) -> Option<SimulationId> {
    let sim_id = match sim_id {
        "3body-moon" => SimulationId::ThreeBodyMoon,
        _ => todo!(),
    };
    Some(sim_id)
}

// ============================================================================

pub fn get_title_from_sim_id(category: &str, sim_id: &str) -> String {
    match category {
        "newtonian-gravity" => match sim_id {
            "3body-moon" => "star + planet + moon",
            _ => todo!("TODO: define title for simulation-id \"{}\"", sim_id),
        },
        "oscillators" => match sim_id {
            "double-pendulum" => "DOUBLE PENDULUM",
            _ => todo!("TODO: define title for simulation-id \"{}\"", sim_id),
        },
        _ => todo!("TODO: define title for category \"{}\"", category),
    }
    .to_string()
}
