use mxyz_universe::system::System;

/// Interaction "Vector"
#[derive(Debug)]
pub struct InteractionVector {
    pub entries: Vec<Option<bool>>, // pub entries: Vec<Vec<Option<bool>>>,
}
impl InteractionVector {
    pub fn new() -> Self {
        let entries = vec![];
        InteractionVector { entries }
    }
    pub fn init(&mut self, systems: &Vec<System>) {
        for _ in 0..systems.len() {
            self.entries.push(None);
        }
    }
    // TODO auto-add/rm rows/cells on system-add/rm
    // TODO run tests for matrix on system-delete
    // TODO run test for all simulation_variants (initialization)
}

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
