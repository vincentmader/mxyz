pub mod game_of_life;
pub mod ising_field;
pub mod temperature;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum FieldVariant {
    FieldVariantV1,
    GameOfLife,
    IsingSpinField,
}
