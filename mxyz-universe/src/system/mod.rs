// pub mod cellular_automata;
pub mod discrete_field;
// pub mod physical_objects;
pub mod planets;
use crate::integrator::Integrator;
use serde::{Deserialize, Serialize};

/// System Structure
#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
/// System Variant Enumeration
pub enum SystemVariant {
    Planets(planets::Planets),
    // DiscreteField(discrete_field::DiscreteField),
    // PhysicalObjects(physical_objects::PhysicalObjects),
    // CellularAutomata(cellular_automata::CellularAutomata)
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
