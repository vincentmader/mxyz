pub mod collision;
pub mod composed;
pub mod diffusion_v1;
pub mod force;
pub mod game_of_life;
pub mod ising_v1;
// pub mod interaction_variant;
use crate::integrator::integrator_variant::IntegratorVariant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use crate::interaction::interaction_variant::InteractionVariant;
// use crate::system::unsized_system::UnsizedSystem;

/// Interaction
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Interaction {
    pub variant: InteractionVariant,
    pub active: bool,
    pub matrix: HashMap<usize, HashMap<usize, bool>>,
}
impl Interaction {
    pub fn new(variant: InteractionVariant) -> Self {
        Interaction {
            variant,
            active: true,
            matrix: HashMap::new(),
        }
    }
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
