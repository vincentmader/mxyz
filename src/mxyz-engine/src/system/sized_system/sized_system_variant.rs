use crate::system::EntitiesV1;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedSystemVariant {
    EntitiesV1(EntitiesV1),
    Field(SizedFieldVariant),
    Objects(SizedObjectsVariant),
    Network(SizedNetworkVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedFieldVariant {}

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedObjectsVariant {}

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedNetworkVariant {}
