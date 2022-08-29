pub mod ants;
pub mod boids;
pub mod charged_particles;
pub mod electrons;
pub mod massive_particles;
pub mod neutrons;
pub mod objects_variant_v1;
pub mod planets;
pub mod protons;
use objects_variant_v1::ObjectsVariantV1;
use planets::Planets;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ObjectsVariant {
    ObjectsVariantV1(ObjectsVariantV1),
    Planets(Planets),
    ChargedParticles,
    Electrons,
    Protons,
    Neutrons,
    Ants,
    Boids,
}
