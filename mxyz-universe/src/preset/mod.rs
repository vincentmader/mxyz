#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SimulationVariant {
    // Undefined, // TODO
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
