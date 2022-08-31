pub mod entities_v1;
pub mod field;
pub mod objects;
pub mod sized_system;
pub mod unsized_system;
pub use entities_v1::EntitiesV1;
pub use sized_system::sized_system_variant::SizedSystemVariant;
pub use sized_system::SizedSystem;
pub use unsized_system::unsized_system_variant::UnsizedSystemVariant;
pub use unsized_system::UnsizedSystem;
// pub use unsized_system::FieldVariant;
// pub use unsized_system::ObjectsVariant;
// pub use unsized_system::NetworkVariant;
// pub use sized_system::SizedFieldVariant;
// pub use sized_system::SizedObjectsVariant;
// pub use sized_system::SizedNetworkVariant;
// pub use sized_system::{SizedSystem, SizedSystemVariant};
// pub use system::{System, SystemVariant};

// use mxyz_universe::entity::Entity;               NOT ALLOWED IN MXYZ-UNIVERSE
// impl IntoParallelIterator for Vec<Box<dyn Entity>> {}
