use serde::{Deserialize, Serialize};

// TODO: implement integrator variants
// - elastic collision
// - inelastic collision (+ energy dampening factor)
// - wall collisions (?)

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollisionIntegratorVariant {}
