pub mod collision;
pub mod force;
use serde::{Deserialize, Serialize};

// ============================================================================

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Interaction {
    pub variant: InteractionVariant,
}

// ============================================================================

/// Interaction Variant
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum InteractionVariant {
    Force(force::Force),
    Collision(collision::Collision),
    // Diffusion(diffusion::Diffusion),
    // GameOfLife(game_of_life::GameOfLife),
    // Ising(ising::Ising),
    // Composed(Box<dyn InteractionTrait>),
}
