mod entities_v1;
pub mod field;
pub mod objects;
mod sized;
mod system;
pub use entities_v1::EntitiesV1;
pub use sized::SizedFieldVariant;
pub use sized::SizedNetworkVariant;
pub use sized::SizedObjectsVariant;
pub use sized::SizedSystem;
pub use sized::SizedSystemVariant;
pub use system::FieldVariant;
pub use system::NetworkVariant;
pub use system::ObjectsVariant;
pub use system::System;
pub use system::SystemVariant;
// pub use sized_system::{SizedSystem, SizedSystemVariant};
// pub use system::{System, SystemVariant};

// use mxyz_universe::entity::Entity;               NOT ALLOWED IN MXYZ-UNIVERSE
// impl IntoParallelIterator for Vec<Box<dyn Entity>> {}
