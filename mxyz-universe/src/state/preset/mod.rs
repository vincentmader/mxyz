#![allow(unreachable_patterns)]

use crate::system::System;
use mxyz_config::EngineConfig;
use mxyz_config::SimulationVariant;
pub mod three_body_figure_eight;
pub mod three_body_moon;

/// Initialize State & Config
pub fn initialize(
    simulation_variant: Option<SimulationVariant>,
    config: &mut EngineConfig,
) -> Vec<System> {
    let mut systems = vec![];
    match &simulation_variant {
        None => {}
        Some(id) => match id {
            SimulationVariant::ThreeBodyFigureEight => {
                three_body_figure_eight::preset(&mut systems, config)
            }
            SimulationVariant::ThreeBodyMoon => three_body_moon::preset(&mut systems, config),
            _ => todo!(),
        },
    }
    config.simulation_variant = simulation_variant;
    systems
}
