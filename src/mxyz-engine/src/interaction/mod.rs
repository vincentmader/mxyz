pub mod collision;
pub mod composed;
pub mod diffusion_v1;
pub mod force;
pub mod game_of_life;
pub mod interaction_variant;
pub mod ising_v1;
use interaction_variant::InteractionVariant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Interaction
/// - Variant, e.g.: Force, Collision
/// - Interaction Matrix
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Interaction {
    pub variant: InteractionVariant,
    pub matrix: HashMap<usize, HashMap<usize, bool>>,
}
impl Interaction {
    pub fn new(variant: InteractionVariant) -> Self {
        Interaction {
            variant,
            matrix: HashMap::new(),
        }
    }
}
