pub mod collision;
pub mod force;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Entity Integrator
pub enum ObjectIntegratorVariant {
    ForceIntegrator(force::ForceIntegratorVariant),
    FooBaz(foo_baz::FooBazIntegratorVariant),
    Collision(collision::CollisionIntegratorVariant),
}

impl ToString for ObjectIntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            Self::ForceIntegrator(x) => x.to_string(),
            Self::Collision(x) => x.to_string(),
            Self::FooBaz(x) => x.to_string(),
        }
    }
}

pub mod foo_baz {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub enum FooBazIntegratorVariant {
        Boid,
    }
    impl ToString for FooBazIntegratorVariant {
        fn to_string(&self) -> String {
            match self {
                Self::Boid => "boid".into(),
            }
        }
    }
}
