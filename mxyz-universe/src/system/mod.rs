pub mod entities_v1;
pub mod objects;
use crate::entity::Entity;
use crate::integrator::Integrator;
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
}

#[derive(Serialize, Deserialize, Debug)]
/// System Variant Enumeration
pub enum SystemVariant {
    EntitiesV1(EntitiesV1),
    Field(FieldVariant),
    Objects(ObjectsVariant),
    //    // Planets(planets::Planets),
    //    // DiscreteField(discrete_field::DiscreteField),
    //    // PhysicalObjects(physical_objects::PhysicalObjects),
    //    // CellularAutomata(cellular_automata::CellularAutomata)
}
impl std::convert::Into<usize> for SystemVariant {
    fn into(self) -> usize {
        match self {
            // SystemVariant::Planets(_) => 0,
            _ => todo!(),
        }
    }
}

impl std::convert::From<usize> for SystemVariant {
    fn from(system_variant: usize) -> SystemVariant {
        match system_variant {
            0 => {
                todo!()
                // let system = crate::system::planets::Planets::new();
                // let system_variant = SystemVariant::Planets(system);
                // system_variant
            }
            _ => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FieldVariant {
    FieldVariantV01,
    GameOfLife,
    IsingSpinField,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ObjectsVariant {
    ObjectsVariantV01,
    Planets(crate::system::objects::planets::Planets),
    ChargedParticles,
    Electrons,
    Protons,
    Neutrons,
    Ants,
    Boids,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitiesV1 {
    // pub entities: Vec<Box<dyn Entity>>,
}

//mod testing {
//    use super::SystemVariant;
//    use crate::entity::field::fluid_cell::FluidCell;
//    use crate::entity::Entity;
//    use crate::integrator::Integrator;
//    use crate::system::FieldVariant;
//    use crate::system::ObjectVariant;
//    use serde::{Deserialize, Serialize};

//    pub struct EngineSystem {
//        pub system_id: usize,
//        pub variant: SystemVariant,
//        pub entities: Vec<Box<dyn Entity>>,
//        pub integrators: Vec<Integrator>,
//    }
//    impl Into<TcpSystem> for EngineSystem {
//        fn into(self) -> TcpSystem {
//            let system_id = self.system_id;
//            let entities = match self.variant {
//                // SystemVariant::Field(field_variant) => match field_variant {
//                // FieldVariant::FieldVariantV01 => {
//                //     EntityVector::Field(FieldEntityVector::FieldVariantV01(self.entities))
//                // }
//                // }, // EntityVector::
//                _ => todo!(),
//            };
//            TcpSystem {
//                system_id,
//                entities,
//            }
//        }
//    }

//    pub struct DatabaseSystem {}

//    #[derive(Serialize, Deserialize, Debug)]
//    pub struct TcpSystem {
//        pub system_id: usize,
//        pub entities: EntityVector,
//    }

//    #[derive(Serialize, Deserialize, Debug)]
//    pub enum EntityVector {
//        Field(FieldEntityVector),
//        Object(ObjectEntityVector),
//    }

//    #[derive(Serialize, Deserialize, Debug)]
//    pub enum FieldEntityVector {
//        FieldVariantV01(Vec<Box<FluidCell>>),
//    }

//    #[derive(Serialize, Deserialize, Debug)]
//    pub enum ObjectEntityVector {}
//}
