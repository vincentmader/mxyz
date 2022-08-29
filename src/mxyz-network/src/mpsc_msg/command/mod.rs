use mxyz_engine::preset::SimulationVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    AddEngine(usize, usize, SimulationVariant),
}
