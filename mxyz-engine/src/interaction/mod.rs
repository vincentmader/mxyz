pub mod collision;
pub mod composed;
pub mod diffusion;
pub mod force;
pub mod game_of_life;
pub mod interaction_matrix;
pub mod ising;
mod testing;
use interaction_matrix::InteractionVector;
use mxyz_universe::interaction::InteractionVariant;

/// Interaction
#[derive(Debug)]
pub struct Interaction {
    pub variant: InteractionVariant,
    pub matrix: InteractionVector,
    // pub neighborhoods:Vec<>
    pub active: bool,
}
impl Interaction {
    pub fn new(variant: InteractionVariant) -> Self {
        Interaction {
            variant,
            matrix: InteractionVector::new(),
            active: true,
        }
    }
}
