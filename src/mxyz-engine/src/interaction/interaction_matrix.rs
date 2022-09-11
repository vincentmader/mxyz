use crate::neighborhoods::NeighborhoodVariant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionMatrix(HashMap<usize, HashMap<usize, NeighborhoodVariant>>); // todo: move to vec-vec?;

impl InteractionMatrix {
    pub fn new(nr_of_systems: usize) -> Self {
        // let matrix = vec![]
        InteractionMatrix(HashMap::new())
    }

    pub fn set_neighborhood_variant(
        &mut self,
        system_id: usize,
        other_id: usize,
        neighborhood: NeighborhoodVariant,
    ) {
        let interaction_matrix = &mut self.0;
        let _ = interaction_matrix
            .entry(system_id)
            .or_insert(HashMap::new());
        interaction_matrix
            .get_mut(&system_id)
            .unwrap()
            .insert(other_id, neighborhood);
    }

    pub fn get_neighborhood_variant(
        &self,
        system_id: usize,
        other_id: usize,
    ) -> NeighborhoodVariant {
        match self.0.get(&system_id) {
            Some(x) => match x.get(&other_id) {
                Some(x) => x.clone(), // TODO remove clone
                None => NeighborhoodVariant::None,
            },
            None => NeighborhoodVariant::None,
        }
    }
}
