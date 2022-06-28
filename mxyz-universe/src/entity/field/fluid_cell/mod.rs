use crate::entity::attribute::*;
use crate::entity::Entity;
use crate::system::ToBytes;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct FluidCell {
    velocity: [f64; 3],
    density: f64,
}
impl Entity for FluidCell {}
impl ToBytes for FluidCell {
    fn to_bytes(&self) -> Vec<u8> {
        let bytes = vec![];
        // ... TODO
        bytes
    }
}
impl Force for FluidCell {}
impl Position for FluidCell {}
impl Charge for FluidCell {}
impl Mass for FluidCell {}
impl FluidCell {
    pub fn new(velocity: [f64; 3], density: f64) -> Self {
        FluidCell { velocity, density }
    }
}
impl Velocity for FluidCell {
    fn get_velocity(&self) -> &[f64; 3] {
        &self.velocity
    }
    fn set_velocity(&mut self, velocity: &[f64; 3]) {
        self.velocity = *velocity;
    }
}
impl Density for FluidCell {
    fn get_density(&self) -> f64 {
        self.density
    }

    fn set_density(&mut self, density: f64) {
        self.density = density;
    }
}
