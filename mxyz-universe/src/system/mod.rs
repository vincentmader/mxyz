pub mod cellular_automata;
pub mod discrete_field;
pub mod physical_objects;
pub mod planets;
use serde::{Deserialize, Serialize};

/// System Structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct System {
    pub system_id: usize,
    pub variant: SystemVariant,
}
impl System {
    pub fn new(system_id: usize, variant: SystemVariant) -> Self {
        System { system_id, variant }
    }
    // pub fn next(&self) -> Self {
    //     let system_id = self.system_id;
    //     println!("SYS-{}", system_id,);

    //     let next_variant = match &self.variant {
    //         SystemVariant::Planets(sys) => {
    //             let a = 0;
    //             SystemVariant::Planets(sys.clone()) //TODO
    //         }
    //         SystemVariant::PhysicalObjects(sys) => {
    //             let a = 0;
    //             SystemVariant::PhysicalObjects(sys.clone()) // TODO
    //         }
    //     };
    //     let variant = self.variant.clone();
    //     let next_system = System::new(system_id, variant);
    //     next_system
    // }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// System Variant Enumeration
pub enum SystemVariant {
    Planets(planets::Planets),
    PhysicalObjects(physical_objects::PhysicalObjects),
    CellularAutomata(cellular_automata::CellularAutomata),
}
impl std::convert::Into<usize> for SystemVariant {
    fn into(self) -> usize {
        match self {
            SystemVariant::Planets(planets) => 0,
            _ => todo!(),
        }
    }
}
impl std::convert::From<usize> for SystemVariant {
    fn from(system_variant: usize) -> SystemVariant {
        match system_variant {
            0 => {
                let system = crate::system::planets::Planets::new();
                let system_variant = SystemVariant::Planets(system);
                system_variant
            }
            _ => todo!(),
        }
    }
}
