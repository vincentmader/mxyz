use crate::entity::attribute::*;
use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalObject {}
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub mass: f64,
    pub charge: f64,
}
impl Entity for PhysicalObject {}
impl PhysicalObject {
    pub fn new(mass: f64, position: [f64; 3], velocity: [f64; 3]) -> Self {
        PhysicalObject {
            position,
            velocity,
            mass,
        }
    }
}
impl Mass for PhysicalObject {
    fn get_mass(&self) -> f64 {
        self.mass
    }
    fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }
}
impl Position for PhysicalObject {
    fn get_position(&self) -> &[f64; 3] {
        &self.position
    }
    fn set_position(&mut self, position: &[f64; 3]) {
        self.position = *position;
    }
}
impl Velocity for PhysicalObject {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn set_velocity(&mut self, velocity: &[f64; 3]) {
        self.velocity = *velocity;
    }
}
impl Charge for PhysicalObject {
    fn get_charge(&self) -> f64 {
        self.charge
    }
    fn set_charge(&mut self, charge: f64) {
        self.charge = charge;
    }

}
impl Force for PhysicalObject {}
impl Density for PhysicalObject {}
