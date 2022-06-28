pub mod discrete_field;
pub mod physical_objects;
use super::entity::Entity;

/// System Variant Enumeration
#[derive(Debug, Clone)]
pub enum SystemVariant {
    DiscreteField,
    PhysicalObjects,
}

/// System Structure
#[derive(Debug, Clone)]
pub struct System {
    pub id: usize,
    pub variant: SystemVariant,
    pub entities: Vec<Box<dyn Entity>>,
}
impl System {
    /// Creates a new System Struct Instance
    pub fn new(variant: SystemVariant) -> Self {
        let id = 0; // TODO safe?
        let entities = vec![];
        System {
            id,
            variant,
            entities,
        }
    }
}
impl ToBytes for System {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes = vec![];

        let _system_variant_id = match self.variant {
            SystemVariant::PhysicalObjects => 0,
            SystemVariant::DiscreteField => 1,
            _ => todo!(),
        };

        for entity in self.entities.iter() {
            let _foo = entity.to_bytes();
        }
        //...
        bytes
    }
}
impl System {
    pub fn get_variant_id(system_variant: &SystemVariant) -> usize {
        match system_variant {
            SystemVariant::PhysicalObjects => 0,
            SystemVariant::DiscreteField => 1,
            _ => todo!(),
        }
    }
}

// TODO move else-where
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
pub trait FromBytes {
    fn from_bytes(bytes: Vec<u8>) -> Self;
}
