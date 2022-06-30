use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    // GetStates(Vec<State>), // TODO whole Engine State?
    GetUpdatedStates(usize),
}
