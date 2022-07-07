pub mod collision;
pub mod composed;
pub mod diffusion;
pub mod force;
pub mod game_of_life;
pub mod ising;
mod testing;
use mxyz_universe::interaction::InteractionVariant;
use mxyz_universe::interaction::InteractionVector;

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
