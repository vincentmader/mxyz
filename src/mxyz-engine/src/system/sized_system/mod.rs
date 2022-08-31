use crate::entity::EntityV1;
use crate::integrator::Integrator;
use crate::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
use crate::system::unsized_system::UnsizedSystem;
use crate::system::EntitiesV1;
use serde::{Deserialize, Serialize};
use sized_system_variant::SizedSystemVariant;
use std::fmt::Debug;
pub mod sized_system_variant;

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub struct SizedSystem {
    pub system_id: usize,
    pub variant: SizedSystemVariant,
    pub integrators: Vec<Integrator>,
}
impl SizedSystem {
    /// Creates a new System
    pub fn new(system_id: usize, variant: SizedSystemVariant) -> Self {
        let integrators = vec![];
        SizedSystem {
            system_id,
            variant,
            integrators,
        }
    }
}
/// Conversion: System -> SizedSystem
impl From<UnsizedSystem> for SizedSystem {
    fn from(system: UnsizedSystem) -> SizedSystem {
        let system_id = system.system_id;
        let integrators = system.integrators;
        let variant = match system.variant {
            UnsizedSystemVariant::EntitiesV1 => {
                let mut e = EntitiesV1::new();
                e.entities = system
                    .entities
                    .into_iter()
                    .map(|e| {
                        let mass = e.get_mass();
                        let position = *e.get_position();
                        let velocity = *e.get_velocity();
                        EntityV1 {
                            mass,
                            position,
                            velocity,
                        }
                    })
                    .collect();
                SizedSystemVariant::EntitiesV1(e)
            }
            _ => todo!("Conversion: System -> SizedSystem"),
        };
        SizedSystem {
            system_id,
            variant,
            integrators,
        }
    }
}
