use mxyz_universe::preset::SimulationVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    SaveStatesToDatabase,
    AddEngine(usize, usize, SimulationVariant),
}
