use crate::neighborhoods::NeighborhoodVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionMatrix(pub Vec<Vec<NeighborhoodVariant>>);

impl InteractionMatrix {
    pub fn new(nr_of_systems: usize) -> Self {
        let matrix = (0..nr_of_systems)
            .map(|_| {
                (0..nr_of_systems)
                    .map(|_| NeighborhoodVariant::None)
                    .collect()
            })
            .collect();
        InteractionMatrix(matrix)
    }
}
