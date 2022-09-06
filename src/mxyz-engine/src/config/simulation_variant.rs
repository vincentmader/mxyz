#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SimulationVariant {
    ThreeBodyFigureEight,
    ThreeBodyMoon,
    SymmetricSatellites,
    ChargeInteraction,
    IsingModel,
}
impl std::convert::Into<usize> for SimulationVariant {
    // NOTE: This is only used for to-file engine exports (directory names) at the moment.
    fn into(self) -> usize {
        match self {
            SimulationVariant::ThreeBodyMoon => 0,
            SimulationVariant::ThreeBodyFigureEight => 1,
            SimulationVariant::ChargeInteraction => 2,
            SimulationVariant::IsingModel => 3,
            _ => todo!(),
        }
    }
}
impl From<usize> for SimulationVariant {
    // NOTE: This is not used at the moment.
    fn from(simulation_variant: usize) -> Self {
        match simulation_variant {
            0 => SimulationVariant::ThreeBodyMoon,
            1 => SimulationVariant::ThreeBodyFigureEight,
            2 => SimulationVariant::ChargeInteraction,
            3 => SimulationVariant::IsingModel,
            _ => todo!(),
        }
    }
}
impl From<&str> for SimulationVariant {
    fn from(simulation_variant: &str) -> Self {
        match simulation_variant {
            "3body-moon" => SimulationVariant::ThreeBodyMoon,
            "3body-fig8" => SimulationVariant::ThreeBodyFigureEight,
            "nbody-charge-interaction" => SimulationVariant::ChargeInteraction,
            "ising-model" => SimulationVariant::IsingModel,
            "symmetric-satellites" => SimulationVariant::SymmetricSatellites,
            _ => todo!(),
        }
    }
}
