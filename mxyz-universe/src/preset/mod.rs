#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SimulationVariant {
    ThreeBodyFigureEight,
    ThreeBodyMoon,
}
impl Into<usize> for SimulationVariant {
    fn into(self) -> usize {
        match self {
            SimulationVariant::ThreeBodyMoon => 0,
            SimulationVariant::ThreeBodyFigureEight => 1,
            _ => todo!("define conversion: enum->usize for simulation-variant"),
        }
    }
}
impl From<usize> for SimulationVariant {
    fn from(simulation_variant: usize) -> Self {
        match simulation_variant {
            0 => SimulationVariant::ThreeBodyMoon,
            1 => SimulationVariant::ThreeBodyFigureEight,
            _ => todo!("define conversion: usize->enum for simulation-variant"),
        }
    }
}
impl From<&str> for SimulationVariant {
    fn from(simulation_variant: &str) -> Self {
        match simulation_variant {
            "3body-fig8" => SimulationVariant::ThreeBodyFigureEight,
            _ => todo!("define conversion: str->enum for simulation-variant"),
        }
    }
}
