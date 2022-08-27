use crate::entity::EntityV1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectsVariantV1 {
    pub entities: Vec<EntityV1>,
}
impl ObjectsVariantV1 {
    pub fn new() -> Self {
        let entities = vec![];
        ObjectsVariantV1 { entities }
    }
}
