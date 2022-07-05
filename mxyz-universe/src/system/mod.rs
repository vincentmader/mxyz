use serde::{Deserialize, Serialize};
pub mod cellular_automata;
pub mod discrete_field;
pub mod physical_objects;
pub mod planets;

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
            SystemVariant::Planets(_) => 0,
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
