use crate::entity::Entity;
use crate::integrator::Integrator;

// ============================================================================

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

// ============================================================================

/// System Variant Enumeration
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

// ============================================================================

#[derive(Debug, Clone)]
pub enum FieldVariant {}

#[derive(Debug, Clone)]
pub enum ObjectsVariant {}

#[derive(Debug, Clone)]
pub enum NetworkVariant {}
