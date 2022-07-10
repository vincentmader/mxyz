use crate::integrator::Integrator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SizedSystem {
    pub system_id: usize,
    pub variant: SizedSystemVariant,
    pub integrators: Vec<Integrator>,
}
impl SizedSystem {
    /// Creates a new System
    pub fn new(system_id: usize, variant: SizedSystemVariant) -> Self {
        // let entities = vec![];
        let integrators = vec![];
        SizedSystem {
            system_id,
            variant,
            // entities,
            integrators,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SizedSystemVariant {}
