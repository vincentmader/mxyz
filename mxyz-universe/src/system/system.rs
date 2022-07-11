use crate::entity::Entity;
use crate::integrator::Integrator;

/// System Structure
#[derive(Debug)]
pub struct System {
    pub system_id: usize,
    pub entities: Vec<Box<dyn Entity>>,
    pub variant: SystemVariant,
    pub integrators: Vec<Integrator>,
}

impl System {
    /// Creates a new System
    pub fn new(system_id: usize, variant: SystemVariant) -> Self {
        let entities = vec![];
        let integrators = vec![];
        System {
            system_id,
            variant,
            entities,
            integrators,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SystemVariant {
    EntitiesV1,
    Field(FieldVariant),
    Objects(ObjectsVariant),
    Network(NetworkVariant),
}
impl std::convert::Into<usize> for SystemVariant {
    fn into(self) -> usize {
        match self {
            SystemVariant::EntitiesV1 => 0,
            _ => todo!("Conversion: SystemVariant -> usize"),
        }
    }
}
impl std::convert::From<usize> for SystemVariant {
    fn from(system_variant: usize) -> SystemVariant {
        match system_variant {
            0 => SystemVariant::EntitiesV1,
            // {
            // let system = crate::system::planets::Planets::new();
            // let system_variant = SystemVariant::Planets(system);
            // system_variant
            // }
            _ => todo!("Conversion: usize -> SystemVariant"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FieldVariant {}

#[derive(Debug, Clone)]
pub enum ObjectsVariant {}

#[derive(Debug, Clone)]
pub enum NetworkVariant {}

// ============================================================================

// #[derive(Serialize, Deserialize, Debug, Clone)]
// /// System Variant Enumeration
// pub enum SystemVariant {
//     EntitiesV1(entities_v1::EntitiesV1),
//     Field(field::FieldVariant),
//     Objects(objects::ObjectsVariant),
// }

// ============================================================================

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
