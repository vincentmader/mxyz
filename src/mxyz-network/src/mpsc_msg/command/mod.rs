use mxyz_engine::config::simulation_variant::SimulationVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    AddEngine(usize, usize, SimulationVariant),
}
