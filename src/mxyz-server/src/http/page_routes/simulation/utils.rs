use mxyz_engine::preset::SimulationVariant;

pub fn get_simulation_variant_from_str(simulation_variant: &str) -> Option<SimulationVariant> {
    let simulation_variant = match simulation_variant {
        "3body-moon" => SimulationVariant::ThreeBodyMoon,
        _ => todo!(),
    };
    Some(simulation_variant)
}

// ============================================================================

pub fn get_title_from_simulation_variant(category: &str, simulation_variant: &str) -> String {
    match category {
        "newtonian-gravity" => match simulation_variant {
            "3body-moon" => "star + planet + moon",
            _ => todo!(
                "TODO: define title for simulation-id \"{}\"",
                simulation_variant
            ),
        },
        "oscillators" => match simulation_variant {
            "double-pendulum" => "DOUBLE PENDULUM",
            _ => todo!(
                "TODO: define title for simulation-id \"{}\"",
                simulation_variant
            ),
        },
        _ => todo!("TODO: define title for category \"{}\"", category),
    }
    .to_string()
}
