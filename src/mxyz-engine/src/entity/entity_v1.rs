use crate::entity::attribute::*;
use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityV1 {
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub charge: f64,
}
impl EntityV1 {
    pub fn new(mass: f64, position: [f64; 3], velocity: [f64; 3], charge: f64) -> Self {
        EntityV1 {
            mass,
            position,
            velocity,
            charge,
        }
    }
}
impl Mass for EntityV1 {
    fn get_mass(&self) -> f64 {
        self.mass
    }
    fn set_mass(&mut self, mass: f64) {
        self.mass = mass
    }
}
impl Position for EntityV1 {
    fn get_position(&self) -> &[f64; 3] {
        &self.position
    }
    fn set_position(&mut self, position: &[f64; 3]) {
        self.position = *position
    }
}
impl Velocity for EntityV1 {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn set_velocity(&mut self, velocity: &[f64; 3]) {
        self.velocity = *velocity
    }
}
impl Charge for EntityV1 {
    fn get_charge(&self) -> f64 {
        self.charge
    }
    fn set_charge(&mut self, charge: f64) {
        self.charge = charge
    }
}
impl Density for EntityV1 {}
impl Force for EntityV1 {}
impl Entity for EntityV1 {}
