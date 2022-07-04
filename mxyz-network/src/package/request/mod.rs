use mxyz_universe::preset::SimulationVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    GetUpdatedStates(usize),
    AddEngine(SimulationVariant),
    RemoveEngine(usize),
}
