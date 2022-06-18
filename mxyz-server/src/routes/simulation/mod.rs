use std::collections::HashMap;

extern crate rocket_dyn_templates;
use rocket_dyn_templates::Template;

#[get("/simulations/<category>/<sim_id>")]
pub fn route(category: &str, sim_id: &str) -> Template {
    start_engine(sim_id);

    let title = get_title_from_sim_id(category, sim_id);
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

pub fn start_engine(sim_id: &str) {
    let sim_id = get_sim_id_from_str(sim_id);

    use mxyz_engine::Engine;
    let mut engine = Engine::new();

    engine.init(&sim_id);
    // engine.run();
}
use mxyz_engine::state::preset::SimulationId;
pub fn get_sim_id_from_str(sim_id: &str) -> Option<SimulationId> {
    let sim_id = match sim_id {
        "3body-moon" => SimulationId::ThreeBodyMoon,
        _ => todo!(),
    };
    Some(sim_id)
}

pub fn get_title_from_sim_id(category: &str, sim_id: &str) -> String {
    match category {
        "gravity" => match sim_id {
            "3body-moon" => "3-Body System: Star + Planet + Moon",
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
