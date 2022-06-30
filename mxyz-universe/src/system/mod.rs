pub mod discrete_field;
pub mod physical_objects;
pub mod planets;
use super::entity::Entity;
// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Debug, Clone)]

// /// System Variant Enumeration
// #[derive(Debug, Clone)]
// pub enum SystemVariant {
//     DiscreteField,
//     PhysicalObjects,
// }

// /// System Structure
// #[derive(Debug, Clone)]
// pub struct System {
//     pub id: usize,
//     pub variant: SystemVariant,
//     pub entities: Vec<Box<dyn Entity>>,
// }
// impl System {
//     /// Creates a new System Struct Instance
//     pub fn new(variant: SystemVariant) -> Self {
//         let id = 0; // TODO safe?
//         let entities = vec![];
//         System {
//             id,
//             variant,
//             entities,
//         }
//     }
// }

/// System Structure
#[derive(Debug, Clone)]
pub struct System {
    pub system_id: usize,
    pub variant: SystemVariant,
}
impl System {
    pub fn new(system_id: usize, variant: SystemVariant) -> Self {
        System { system_id, variant }
    }
    pub fn step(self) {
        match self.variant {
            SystemVariant::Planets(sys) => {}
        }
    }
}
#[derive(Debug, Clone)]
/// System Variant Enumeration
pub enum SystemVariant {
    Planets(planets::Planets),
}
