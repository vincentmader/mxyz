use crate::entity::object::planet::Planet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Planets {
    pub entities: Vec<Planet>,
}
impl Planets {
    pub fn new() -> Self {
        let entities = vec![];
        Planets { entities }
    }
}
