use crate::entity::EntityV1;
use crate::integrator::Integrator;
use crate::system::system::System;
use crate::system::system::SystemVariant;
use crate::system::EntitiesV1;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
impl From<System> for SizedSystem {
    fn from(system: System) -> SizedSystem {
        let system_id = system.system_id;
        let integrators = system.integrators;
        let variant = match system.variant {
            SystemVariant::EntitiesV1 => {
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

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedSystemVariant {
    EntitiesV1(EntitiesV1),
    Field(SizedFieldVariant),
    Objects(SizedObjectsVariant),
    Network(SizedNetworkVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedFieldVariant {}

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedObjectsVariant {}

#[derive(Debug, Clone, Serialize, Deserialize)] // TODO remove clone again
pub enum SizedNetworkVariant {}
