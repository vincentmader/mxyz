use crate::system::sized_system::SizedSystem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)] // TODO remove clone again
pub struct SizedState {
    pub state_id: usize,
    pub systems: Vec<SizedSystem>,
}
impl SizedState {
    pub fn new(state_id: usize) -> Self {
        let systems = vec![];
        SizedState { state_id, systems }
    }
}
