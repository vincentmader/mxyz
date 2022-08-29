pub mod coulomb_interaction;
pub mod lennard_jones_interaction;
pub mod newtonian_gravity;

use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ForceVariant {
    Coulomb,
    NewtonianGravity,
    LennardJones,
    Hooke,
    Cohesion,
    Avoidance,
    Alignment,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Force {
    pub variant: ForceVariant,
}
impl Force {
    pub fn new(variant: ForceVariant) -> Self {
        Force { variant }
    }
    pub fn apply(&self, entity: &Box<dyn Entity>) {}
}

// fn _force_coulomb(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
//     println!("\t\tCOULOMB");
//     let (q1, q2) = (entity.get_charge(), other.get_charge());
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const K: f64 = 1.; // TODO
//     let force = K * (q1 * q2) / (r * r);
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
// fn _force_newton(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
//     println!("\t    NEWTON");
//     let (m1, m2) = (entity.get_mass(), other.get_mass());
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     let u: Vec<f64> = u.iter().map(|i| i / r).collect();
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const G: f64 = 1.; // TODO
//     let force = G * (m1 * m2) / (r * r);
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
// fn _force_lennard_jones(
//     entity: &Box<dyn PhysicalObject>,
//     other: &Box<dyn PhysicalObject>,
// ) -> [f64; 3] {
//     println!("\t    LENNARD-JONES");
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const E: f64 = 1.;
//     const S: f64 = 1.;
//     let force = E * ((S / r).powf(13.) - (S / r).powf(7.));
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
// fn _force_hooke(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
//     println!("\t    HOOKE");
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const K: f64 = 1.; // TODO
//     let force = K * r;
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
