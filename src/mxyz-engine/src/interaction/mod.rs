pub mod collision;
pub mod composed;
pub mod diffusion_v1;
pub mod force;
pub mod game_of_life;
pub mod ising_v1;
use crate::system::System;
use serde::{Deserialize, Serialize};

// ============================================================================

/// Interaction
#[derive(Debug, Clone, Deserialize, Serialize)]
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

// ============================================================================

/// Interaction "Vector" (Interaction Matrix Row)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractionVector {
    pub entries: Vec<Option<bool>>, // pub entries: Vec<Vec<Option<bool>>>,
}
impl InteractionVector {
    /// Creates a new Interaction Matrix Row
    pub fn new() -> Self {
        let entries = vec![];
        InteractionVector { entries }
    }
    /// Initialize Interaction Matrix Row
    pub fn init(&mut self, systems: &Vec<System>) {
        for _ in 0..systems.len() + 1 {
            self.entries.push(None);
        }
    }
    // TODO auto-add/rm rows/cells on system-add/rm

    // TODO run tests for matrix on system-delete

    // TODO run test for all simulation_variants (initialization)
}

// use crate::entity::Entity;
// pub fn foo(entities: Vec<Box<dyn Entity>>) {}

// /// Interaction Matrix
// #[derive(Debug)]
// pub struct InteractionMatrix {
//     pub rows: Vec<InteractionMatrixRow>, // pub entries: Vec<Vec<Option<bool>>>,
// }
// impl InteractionMatrix {
//     pub fn new() -> Self {
//         let rows = vec![];
//         InteractionMatrix { rows }
//     }
//     pub fn init(&mut self, systems: &Vec<System>) {
//         for _ in 0..systems.len() {
//             let mut row = InteractionMatrixRow::new();
//             row.init(&systems);
//             self.rows.push(row);
//         }
//     }
//     // TODO auto-add/rm rows/cells on system-add/rm
//     // TODO run tests for matrix on system-delete
//     // TODO run test for all simulation_variants (initialization)
// }

// #[derive(Debug)]
// pub struct InteractionMatrixRow {
//     pub entries: Vec<InteractionMatrixEntry>,
// }
// impl InteractionMatrixRow {
//     pub fn new() -> Self {
//         let entries = vec![];
//         InteractionMatrixRow { entries }
//     }
//     pub fn init(&mut self, systems: &Vec<System>) {
//         for _ in 0..systems.len() {
//             self.entries.push(InteractionMatrixEntry::new());
//         }
//     }
// }

// #[derive(Debug)]
// pub struct InteractionMatrixEntry {
//     pub active: Option<bool>,
// }
// impl InteractionMatrixEntry {
//     pub fn new() -> Self {
//         let active = None;
//         InteractionMatrixEntry { active }
//     }
// }
