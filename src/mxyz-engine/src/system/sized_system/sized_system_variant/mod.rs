pub mod sized_field_variant;
pub mod sized_objects_variant;
use crate::system::entities_v1::EntitiesV1;
use serde::{Deserialize, Serialize};
use sized_field_variant::SizedFieldVariant;
use sized_objects_variant::SizedObjectsVariant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SizedSystemVariant {
    EntitiesV1(EntitiesV1),
    Field(SizedFieldVariant),
    Objects(SizedObjectsVariant),
}
