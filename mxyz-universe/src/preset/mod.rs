#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};
pub mod three_body_figure_eight;
pub mod three_body_moon;
use crate::preset::SimulationVariant;
use crate::state::State;
use mxyz_config::EngineConfig;

/// Initialize State & Config
pub fn initialize(simulation_variant: Option<SimulationVariant>, cfg: &mut EngineConfig) -> State {
    let mut state = State::new(0);
    let mut systems = vec![];
    match simulation_variant {
        Some(simulation_variant) => match simulation_variant {
            SimulationVariant::ThreeBodyMoon => {
                three_body_moon::preset(&mut systems, cfg);
                state.systems = systems;
                state
            }
            _ => todo!(),
        },
        None => todo!("handle this earlier? (in str->enum sim-var conversion)"),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SimulationVariant {
    ThreeBodyFigureEight,
    ThreeBodyMoon,
}
impl std::convert::Into<usize> for SimulationVariant {
    fn into(self) -> usize {
        match self {
            SimulationVariant::ThreeBodyMoon => 0,
            SimulationVariant::ThreeBodyFigureEight => 1,
            _ => todo!(),
        }
    }
}
impl From<usize> for SimulationVariant {
    fn from(simulation_variant: usize) -> Self {
        if simulation_variant == 0 {
            SimulationVariant::ThreeBodyMoon
        } else if simulation_variant == 1 {
            SimulationVariant::ThreeBodyFigureEight
        } else {
            todo!()
        }
    }
}
