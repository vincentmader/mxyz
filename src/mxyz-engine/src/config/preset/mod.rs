#![allow(unreachable_patterns)]
pub mod boids;
pub mod charge_interaction;
pub mod ising_model;
pub mod symmetric_satellites;
pub mod three_body_figure_eight;
pub mod three_body_moon;
use crate::config::simulation_variant::SimulationVariant;
use crate::config::EngineConfig;
use crate::state::UnsizedState;

/// Initialize State & Config
pub fn get_initial_state(
    simulation_variant: Option<SimulationVariant>,
    cfg: &mut EngineConfig,
) -> UnsizedState {
    let mut state = UnsizedState::new(0);
    let mut systems = vec![];
    match simulation_variant {
        Some(simulation_variant) => match simulation_variant {
            SimulationVariant::ThreeBodyMoon => three_body_moon::preset(&mut systems, cfg),
            SimulationVariant::Boids => boids::preset(&mut systems, cfg),
            // SimulationVariant::ChargeInteraction => charge_interaction::preset(&mut systems, cfg),
            // SimulationVariant::IsingModel => ising_model::preset(&mut systems, cfg),
            // SimulationVariant::SymmetricSatellites => {
            //     symmetric_satellites::preset(&mut systems, cfg)
            // }
            _ => todo!("{:?}", simulation_variant),
        },
        None => todo!("handle this earlier? (in str->enum sim-var conversion)"),
    }
    state.systems = systems;
    state
}
