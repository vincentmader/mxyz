use crate::integrator::Integrator;

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
