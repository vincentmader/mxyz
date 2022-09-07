use serde::{Deserialize, Serialize};

// TODO: implement integrator variants
// - elastic collision
// - inelastic collision (+ energy dampening factor)
// - wall collisions (?)

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CollisionIntegratorVariant {
    ElasticCollision,
    InelasticCollision,
}

impl ToString for CollisionIntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            ElasticCollision => "Elastic Collision".into(),
            InelasticCollision => "Inelastic Collision".into(),
        }
    }
}
