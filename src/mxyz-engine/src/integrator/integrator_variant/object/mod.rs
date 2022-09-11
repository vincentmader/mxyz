pub mod collision;
pub mod force;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ObjectIntegratorVariant {
    Force(force::ForceIntegratorVariant),
    Collision(collision::CollisionIntegratorVariant),
    FooBaz(foo_baz::FooBazIntegratorVariant),
}

impl ToString for ObjectIntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Force(x) => x.to_string(),
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
