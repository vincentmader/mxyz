pub mod fluid_cell;
use crate::entity::attribute::*;

pub trait DiscreteFieldCell: Density + Velocity + Force {}

// /// Entity Trait: Discrete Field Cell
// pub trait DiscreteFieldCell: DiscreteFieldCellClone + Density + Velocity + Force {}
// pub trait DiscreteFieldCellClone {
//     fn clone_box(&self) -> Box<dyn DiscreteFieldCell>;
// }
// impl<T> DiscreteFieldCellClone for T
// where
//     T: 'static + DiscreteFieldCell + Clone,
// {
//     fn clone_box(&self) -> Box<dyn DiscreteFieldCell> {
//         Box::new(self.clone())
//     }
// }
// impl Clone for Box<dyn DiscreteFieldCell> {
//     fn clone(&self) -> Box<dyn DiscreteFieldCell> {
//         self.clone_box()
//     }
// }
