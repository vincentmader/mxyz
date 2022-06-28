pub mod attribute;
pub mod field;
pub mod object;
use crate::system::ToBytes;
use attribute::*;
// use serde::{Deserialize, Serialize};

/// Entity Trait
pub trait Entity:
    EntityClone
    // + Serialize
    // + Deserialize
    + Mass
    + Position
    + Velocity
    + Charge
    + Density
    + Force
    + ToBytes
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
