pub mod interaction_variant;
use interaction_variant::InteractionVariant;
use serde::{Deserialize, Serialize};

/// Interaction
/// - Variant, e.g.: Force, Collision
/// - Activity: true/false
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Interaction {
    pub variant: InteractionVariant,
    pub active: bool,
}
impl Interaction {
    pub fn new(variant: InteractionVariant) -> Self {
        Interaction {
            active: true,
            variant,
        }
    }
}
