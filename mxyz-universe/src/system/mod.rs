pub mod entities_v1;
pub mod field;
pub mod objects;
use crate::entity::Entity;
use crate::entity::EntityV1;
use crate::integrator::Integrator;
use field::FieldVariant;
use objects::ObjectsVariant;
use serde::{Deserialize, Serialize};

/// System Structure
#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    pub system_id: usize,
    pub variant: SystemVariant,
    pub integrators: Vec<Integrator>,
}

impl System {
    pub fn new(system_id: usize, variant: SystemVariant) -> Self {
        let integrators = vec![];
        System {
            system_id,
            variant,
            integrators,
        }
    }

    // pub fn entities(&self) -> &Vec<Box<dyn Entity>> {
    //     match &self.variant {
    //         SystemVariant::EntitiesV1(system) => todo!(),
    //         SystemVariant::Field(field_variant) => match field_variant {
    //             FieldVariant::FieldVariantV1 => todo!(),
    //             FieldVariant::GameOfLife => todo!(),
    //             FieldVariant::IsingSpinField => todo!(),
    //             _ => todo!(),
    //         },
    //         SystemVariant::Objects(objects_variant) => match objects_variant {
    //             ObjectsVariant::ObjectsVariantV1(objects) => &objects
    //                 .entities
    //                 .into_iter()
    //                 .map(|x| Box::<dyn Entity>::from(Box::from(x)))
    //                 .collect(),

    //             ObjectsVariant::Ants => todo!(),
    //             ObjectsVariant::Planets(planets) => &planets
    //                 .entities
    //                 .into_iter()
    //                 .map(|x| Box::<dyn Entity>::from(Box::from(x)))
    //                 .collect(),
    //             // as Vec<Box<dyn Entity>>,
    //             ObjectsVariant::Boids => todo!(),
    //             ObjectsVariant::ChargedParticles => todo!(),
    //             ObjectsVariant::Electrons => todo!(),
    //             ObjectsVariant::Neutrons => todo!(),
    //             ObjectsVariant::Protons => todo!(),
    //             _ => todo!(),
    //         },
    //         _ => todo!(),
    //     }
    // }
}

// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
/// System Variant Enumeration
pub enum SystemVariant {
    EntitiesV1(EntitiesV1),
    Field(FieldVariant),
    Objects(ObjectsVariant),
}
impl std::convert::Into<usize> for SystemVariant {
    fn into(self) -> usize {
        match self {
            // SystemVariant::Planets(_) => 0,
            _ => todo!("conversion from SystemVariant enum to usize"),
        }
    }
}
impl std::convert::From<usize> for SystemVariant {
    fn from(system_variant: usize) -> SystemVariant {
        match system_variant {
            // 0 => {
            // let system = crate::system::planets::Planets::new();
            // let system_variant = SystemVariant::Planets(system);
            // system_variant
            // }
            _ => todo!("conversion from usize enum to SystemVariant"),
        }
    }
}

// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitiesV1 {
    pub entities: Vec<EntityV1>,
}
