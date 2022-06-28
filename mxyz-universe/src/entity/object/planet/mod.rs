use crate::entity::attribute::*;
use crate::entity::Entity;
use crate::system::ToBytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Planet {
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub mass: f64,
}
impl Entity for Planet {}
impl ToBytes for Planet {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes = vec![];
        // ... TODO
        bytes
    }
}
impl Planet {
    pub fn new(mass: f64, position: [f64; 3], velocity: [f64; 3]) -> Self {
        Planet {
            position,
            velocity,
            mass,
        }
    }
}
impl Mass for Planet {
    fn get_mass(&self) -> f64 {
        self.mass
    }
    fn set_mass(&mut self, mass: f64) {
        self.mass = mass;
    }
}
impl Position for Planet {
    fn get_position(&self) -> &[f64; 3] {
        &self.position
    }
    fn set_position(&mut self, position: &[f64; 3]) {
        self.position = *position;
    }
}
impl Velocity for Planet {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn set_velocity(&mut self, velocity: &[f64; 3]) {
        self.velocity = *velocity;
    }
}
impl Charge for Planet {}
impl Force for Planet {}
impl Density for Planet {}
