use crate::entity::entity_v1::EntityV1;
use crate::entity::Entity;
use crate::integrator::Integrator;
use crate::system::sized_system::sized_system_variant::SizedSystemVariant;
use crate::system::sized_system::SizedSystem;
use crate::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
pub mod unsized_system_variant;

/// Unsized System
#[derive(Debug, Clone)]
pub struct UnsizedSystem {
    pub system_id: usize,
    pub entities: Vec<Box<dyn Entity>>,
    pub variant: UnsizedSystemVariant,
    pub integrators: Vec<Integrator>, // TODO move integrators to config
}
impl UnsizedSystem {
    /// Create a new System.
    pub fn new(system_id: usize, variant: UnsizedSystemVariant) -> Self {
        let entities = vec![];
        let integrators = vec![];
        UnsizedSystem {
            system_id,
            variant,
            entities,
            integrators,
        }
    }
}
/// Convert from SizedSystem to UnsizedSystem.
impl From<&SizedSystem> for UnsizedSystem {
    fn from(system: &SizedSystem) -> UnsizedSystem {
        let system_id = system.system_id;
        let variant = (&system.variant).into();
        let integrators = system.integrators.clone(); // TODO move integrators to config

        let mut entities: Vec<Box<dyn Entity>> = vec![];
        match &system.variant {
            SizedSystemVariant::EntitiesV1(sys) => {
                let _: Vec<()> = sys
                    .entities
                    .iter()
                    .map(|ent| {
                        let m = ent.mass;
                        let pos = ent.position;
                        let vel = ent.velocity;
                        let q = ent.charge;
                        let ent = EntityV1::new(m, pos, vel, q);
                        entities.push(Box::new(ent));
                    })
                    .collect();
            }
            _ => todo!(),
        };
        UnsizedSystem {
            system_id,
            entities,
            variant,
            integrators,
        }
    }
}
