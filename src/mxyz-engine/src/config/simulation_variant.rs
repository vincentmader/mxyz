#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SimulationVariant {
    ThreeBodyFigureEight,
    ThreeBodyMoon,
    SymmetricSatellites,
    ChargeInteraction,
    IsingModel,
    Boids,
}
impl std::convert::Into<usize> for SimulationVariant {
    // NOTE: This is only used for to-file engine exports (directory names) at the moment.
    fn into(self) -> usize {
        match self {
            Self::ThreeBodyMoon => 0,
            Self::ThreeBodyFigureEight => 1,
            Self::ChargeInteraction => 2,
            Self::IsingModel => 3,
            Self::Boids => 4,
            _ => todo!(),
        }
    }
}
impl From<usize> for SimulationVariant {
    // NOTE: This is not used at the moment.
    fn from(simulation_variant: usize) -> Self {
        match simulation_variant {
            0 => Self::ThreeBodyMoon,
            1 => Self::ThreeBodyFigureEight,
            2 => Self::ChargeInteraction,
            3 => Self::IsingModel,
            4 => Self::Boids,
            _ => todo!(),
        }
    }
}
impl From<&str> for SimulationVariant {
    fn from(simulation_variant: &str) -> Self {
        match simulation_variant {
            "3body-moon" => Self::ThreeBodyMoon,
            "3body-fig8" => Self::ThreeBodyFigureEight,
            "nbody-charge-interaction" => Self::ChargeInteraction,
            "ising-model" => Self::IsingModel,
            "symmetric-satellites" => Self::SymmetricSatellites,
            "nbody-boids" => Self::Boids,
            _ => todo!(),
        }
    }
}
