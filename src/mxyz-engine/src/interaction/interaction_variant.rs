use serde::{Deserialize, Serialize};

/// Interaction Variant
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InteractionVariant {
    Force(crate::interaction::force::Force),
    Collision(crate::interaction::collision::Collision),
    // Diffusion(diffusion::Diffusion),
    // GameOfLife(game_of_life::GameOfLife),
    // Ising(ising::Ising),
    // Composed(Box<dyn InteractionTrait>),
}
impl ToString for InteractionVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Force(x) => x.variant.to_string(),
            Self::Collision(_x) => "Collision".into(),
        }
    }
}
