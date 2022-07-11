pub mod attribute;
pub mod field;
pub mod object;
use attribute::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// ============================================================================

/// Entity Trait
pub trait Entity:
    Mass + Position + Velocity + Charge + Density + Force + EntityClone + Debug + Send
{
}

pub trait EntityClone {
    fn clone_box(&self) -> Box<dyn Entity>;
}
impl<T> EntityClone for T
where
    T: 'static + Entity + Clone,
{
    fn clone_box(&self) -> Box<dyn Entity> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn Entity> {
    fn clone(&self) -> Box<dyn Entity> {
        self.clone_box()
    }
}

// ============================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntityV1 {
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
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
impl Charge for EntityV1 {}
impl Density for EntityV1 {}
impl Force for EntityV1 {}
impl Entity for EntityV1 {}
// impl EntityClone for Entity {}
