pub mod planet;
use crate::entity::attribute::*;

pub trait PhysicalObject: Mass + Position + Velocity + Charge {}

// /// Entity Trait: Physical Object
// pub trait PhysicalObject: PhysicalObjectClone + Position + Velocity + Mass + Charge {}
// pub trait PhysicalObjectClone {
//     fn clone_box(&self) -> Box<dyn PhysicalObject>;
// }
// impl<T> PhysicalObjectClone for T
// where
//     T: 'static + PhysicalObject + Clone,
// {
//     fn clone_box(&self) -> Box<dyn PhysicalObject> {
//         Box::new(self.clone())
//     }
// }
// impl Clone for Box<dyn PhysicalObject> {
//     fn clone(&self) -> Box<dyn PhysicalObject> {
//         self.clone_box()
//     }
// }
