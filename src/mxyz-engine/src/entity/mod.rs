pub mod attribute;
pub mod entity_v1;
use attribute::*;
use entity_v1::EntityV1;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// ============================================================================

/// Entity Trait
pub trait Entity:
    Mass + Position + Velocity + Charge + Density + Force + EntityClone + Debug + Send + Sync
//                                                                                        ^ added this, needed?
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
