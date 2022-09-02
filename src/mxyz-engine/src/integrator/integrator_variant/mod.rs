pub mod field;
pub mod network;
pub mod object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IntegratorVariant {
    Field(field::FieldIntegratorVariant),
    Object(object::ObjectIntegratorVariant),
    Network(network::NetworkIntegratorVariant),
}
