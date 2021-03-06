// use mxyz_network::package::Package;
// use std::sync::mpsc::Receiver;

pub struct ClientConfig {
    pub client_id: usize,
    pub frame_id: (usize, usize),
    // pub rx: Receiver<Package>,
}
impl ClientConfig {
    pub fn new(
        client_id: usize,
        // rx: Receiver<Package>
    ) -> Self {
        let frame_id = (0, usize::MAX);
        ClientConfig {
            client_id,
            frame_id,
            // rx,
        }
    }
}

pub struct RendererConfig {}

use mxyz_universe::preset::SimulationVariant;

pub struct EngineConfig {
    pub step_id: (usize, usize),
    pub simulation_variant: Option<SimulationVariant>,
    // pub systems: Vec<SystemConfig>,
    // pub interactions: Vec<Interaction>,
    // pub integrators: Vec<Vec<Integrator>>,
    // pub constants: Constants,
    pub export_variant: ExportVariant,
    pub last_export_step_id: Option<usize>,
    pub nr_of_steps_between_exports: usize,
}
impl EngineConfig {
    pub fn new() -> Self {
        // let systems = vec![];
        // let interactions = vec![];
        // let integrators = vec![];
        let step_id = (0, usize::MAX);
        // let constants = Constants::new();
        let export_variant = ExportVariant::ToDatabase;
        let last_export_step_id = None;
        let nr_of_steps_between_exports = 100;
        let simulation_variant = None;
        EngineConfig {
            // systems,
            // interactions,
            // integrators,
            step_id,
            // constants,
            export_variant,
            last_export_step_id,
            nr_of_steps_between_exports,
            simulation_variant,
        }
    }
}

// pub struct EngineConfig {
//     pub step_id: (usize, usize),
// }
// impl EngineConfig {
//     pub fn new() -> Self {
//         let step_id = (0, usize::MAX);
//         // let interactions = vec![];
//         // let integrators = vec![];
//         // let export_variant = ExportVariant::ToDatabase;
//         // let last_export_step_id = None;
//         // let nr_of_steps_between_exports = 100;
//         // let simulation_variant = None;
//         EngineConfig {
//             step_id,
//             // systems,
//             // interactions,
//             // integrators,
//             // export_variant,
//             // last_export_step_id,
//             // nr_of_steps_between_exports,
//             // simulation_variant,
//         }
//     }
// }

pub enum ExportVariant {
    ToDatabase,
    ToFile,
}

// // TODO move else-where
// pub struct InteractionMatrix {
//     rows: Vec<InteractionMatrixRow>,
// }
// impl InteractionMatrix {
//     pub fn new() -> Self {
//         let rows = vec![];
//         InteractionMatrix { rows }
//     }
//     pub fn add_row(&mut self) {
//         let row = InteractionMatrixRow::new(self.rows.len());
//         self.rows.push(row);
//         for row in self.rows.iter_mut() {
//             row.entries.push(vec![]);
//         }
//     }
//     pub fn remove_row(&mut self, row_id: usize) {
//         self.rows.remove(row_id);
//         for row in self.rows.iter_mut() {
//             row.entries.remove(row_id);
//         }
//     }
//     pub fn get_interactions(&self, system: usize, other: usize) -> &Vec<Interaction> {
//         self.rows.get(system).unwrap().entries.get(other).unwrap()
//     }
//     pub fn activate_interaction(&mut self, system: usize, other: usize, interaction: &Interaction) {
//         // let interactions = &mut self.rows[system].entries[other];
//         // if !interactions.contains(&interaction) {
//         //     interactions.push(interaction);
//         // }
//     }
//     pub fn deactivate_interaction(
//         &mut self,
//         system: usize,
//         other: usize,
//         interaction: &Interaction,
//     ) {
//         // let interactions = &mut self.rows[system].entries[other];
//         // interactions.retain(|i| i != interaction);
//     }
// }

// pub struct InteractionMatrixRow {
//     entries: Vec<Vec<Interaction>>,
// }
// impl InteractionMatrixRow {
//     pub fn new(nr_of_entries: usize) -> Self {
//         let mut entries = vec![];
//         for _ in 0..nr_of_entries {
//             entries.push(vec![]);
//         }
//         InteractionMatrixRow { entries }
//     }
// }

// pub struct Constants {
//     _g: f64, // Newton's gravitational constant
// }
// impl Constants {
//     pub fn new() -> Self {
//         let g = 1.;
//         Constants { _g: g }
//     }
// }
