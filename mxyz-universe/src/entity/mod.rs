pub mod attribute;
pub mod field;
pub mod object;
use attribute::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// ============================================================================

/// Entity Trait
pub trait Entity: Mass + Position + Velocity + Charge + Density + Force + Debug + Send {}

// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityV1 {}
impl Mass for EntityV1 {}
impl Position for EntityV1 {}
impl Velocity for EntityV1 {}
impl Charge for EntityV1 {}
impl Density for EntityV1 {}
impl Force for EntityV1 {}
impl Entity for EntityV1 {}
