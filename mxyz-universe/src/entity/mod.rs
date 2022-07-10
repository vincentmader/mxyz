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
pub struct EntityV1 {}
impl Mass for EntityV1 {}
impl Position for EntityV1 {}
impl Velocity for EntityV1 {}
impl Charge for EntityV1 {}
impl Density for EntityV1 {}
impl Force for EntityV1 {}
impl Entity for EntityV1 {}
