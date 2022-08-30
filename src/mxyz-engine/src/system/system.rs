use crate::entity::Entity;
use crate::entity::EntityV1;
use crate::integrator::Integrator;
use crate::system::SizedSystem;
use crate::system::SizedSystemVariant;

// ============================================================================

/// System Structure
#[derive(Debug, Clone)]
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
impl From<&SizedSystem> for System {
    fn from(system: &SizedSystem) -> System {
        let system_id = system.system_id;
        let variant = (&system.variant).into();
        let integrators = system.integrators.clone(); //TODO

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
                        let ent = EntityV1::new(m, pos, vel);
                        entities.push(Box::new(ent));
                    })
                    .collect();
            }
            _ => todo!(),
        };
        System {
            system_id,
            entities,
            variant,
            integrators,
        }
    }
}

// ============================================================================

/// System Variant Enumeration
#[derive(Debug, Clone)]
pub enum SystemVariant {
    EntitiesV1,
    Field(FieldVariant),
    Objects(ObjectsVariant),
    Network(NetworkVariant),
}
impl Into<usize> for SystemVariant {
    fn into(self) -> usize {
        match self {
            SystemVariant::EntitiesV1 => 0,
            _ => todo!("Conversion: SystemVariant -> usize"),
        }
    }
}
impl From<usize> for SystemVariant {
    fn from(system_variant: usize) -> SystemVariant {
        match system_variant {
            0 => SystemVariant::EntitiesV1,
            _ => todo!("Conversion: usize -> SystemVariant"),
        }
    }
}
impl From<&SizedSystemVariant> for SystemVariant {
    fn from(system_variant: &SizedSystemVariant) -> SystemVariant {
        match system_variant {
            SizedSystemVariant::EntitiesV1(_) => SystemVariant::EntitiesV1,
            _ => todo!(),
        }
    }
}

// ============================================================================

#[derive(Debug, Clone)]
pub enum FieldVariant {}

#[derive(Debug, Clone)]
pub enum ObjectsVariant {}

#[derive(Debug, Clone)]
pub enum NetworkVariant {}
