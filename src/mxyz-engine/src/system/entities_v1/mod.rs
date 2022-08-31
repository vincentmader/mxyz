use crate::entity::entity_v1::EntityV1;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug)]
/// System: EntitiesV1
pub struct EntitiesV1 {
    pub entities: Vec<EntityV1>,
}
impl EntitiesV1 {
    pub fn new() -> Self {
        let entities = vec![];
        EntitiesV1 { entities }
    }
}
