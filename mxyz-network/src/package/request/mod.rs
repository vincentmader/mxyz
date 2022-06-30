use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Request {
    // GetStates(Vec<State>), // TODO whole Engine State?
    GetUpdatedStates(usize),
}
