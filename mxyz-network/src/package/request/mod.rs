use mxyz_config::SimulationVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Adds Client
    AddClient,
    /// Adds Engine with given Client-ID & SimulationVariant
    AddEngine(usize, SimulationVariant),
    /// Removes Engine
    RemoveEngine(usize),
    /// Gets States since last Update
    GetUpdatedStates(usize, usize), // engine-id, state-id
}
