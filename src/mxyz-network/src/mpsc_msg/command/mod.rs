use mxyz_engine_universe::preset::SimulationVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    AddEngine(usize, usize, SimulationVariant),
}
