use crate::entity::Entity;

// TODO move else-where
const G: f64 = 1.; // Newton Gravity
const K: f64 = 1.; // Coulomb

#[derive(Debug)]
pub enum ForceVariant {
    Coulomb,
    NewtonianGravity,
    LennardJones,
    Hooke,
    Cohesion,
    Avoidance,
    Alignment,
}

#[derive(Debug)]
pub struct Force {
    pub variant: ForceVariant,
}
impl Force {
    pub fn new(variant: ForceVariant) -> Self {
        Force { variant }
    }
    pub fn calculate_from<T: Entity, O: Entity>(
        &self,
        // entity: &Box<dyn Entity>,
        // other: &Box<dyn Entity>,
        entity: &T,
        other: &O,
    ) -> [f64; 3] {
        // TODO move match up? (not needed for every entity, is it?)
        let force = match self.variant {
            ForceVariant::NewtonianGravity => {
                let (m1, m2) = (entity.get_mass(), other.get_mass());
                let (x1, x2) = (entity.get_position(), other.get_position());
                let rel_x = [x2[0] - x1[0], x2[1] - x1[1], x2[2] - x1[2]];
                let dist = rel_x.iter().map(|dx| dx * dx).sum::<f64>();
                let unit = [rel_x[0] / dist, rel_x[1] / dist, rel_x[1] / dist];
                let force = G * (m1 * m2) / dist.powf(2.);
                let force = [unit[0] * force, unit[1] * force, unit[2] * force];
                force
            }
            ForceVariant::Coulomb => {
                let (q1, q2) = (entity.get_charge(), other.get_charge());
                let (x1, x2) = (entity.get_position(), other.get_position());
                let rel_x = [x2[0] - x1[0], x2[1] - x1[1], x2[2] - x1[2]];
                let dist = rel_x.iter().map(|dx| dx * dx).sum::<f64>();
                let unit = [rel_x[0] / dist, rel_x[1] / dist, rel_x[1] / dist];
                let force = K * (q1 * q2) / dist.powf(2.);
                let force = [unit[0] * force, unit[1] * force, unit[2] * force];
                force
            }

            _ => todo!(),
        };
        force
    }
    // pub fn apply_to_objects_from_objects(
    //     &self,
    //     entities: &mut Vec<Box<dyn PhysicalObject>>,
    //     others: &Vec<Box<dyn PhysicalObject>>,
    //     integrator: &Integrator,
    //     self_interaction: bool,
    // ) {
    //     let force_getter = match self.variant {
    //         ForceVariant::NewtonianGravity => force_newton,
    //         ForceVariant::Coulomb => force_coulomb,
    //         ForceVariant::Hooke => force_hooke,
    //         ForceVariant::LennardJones => force_lennard_jones,
    //         _ => todo!(),
    //     };

    //     let integrator = match integrator.variant {
    //         IntegratorVariant::EulerExplicit => integrator::euler_explicit,
    //         IntegratorVariant::RungeKutta4 => integrator::runge_kutta_4,
    //         IntegratorVariant::Verlet => integrator::verlet,
    //         _ => todo!(),
    //     };

    //     for (entity_id, mut entity) in entities.iter_mut().enumerate() {
    //         for (other_id, other) in others.iter().enumerate() {
    //             println!("\t{} - {}", entity_id, other_id);
    //             if self_interaction {
    //                 if entity_id == other_id {
    //                     println!("\t    skip");
    //                     continue;
    //                 }
    //             }
    //             integrator(&mut entity, &other, force_getter);
    //         }
    //     }
    // }
}

// fn _force_coulomb(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
//     println!("\t\tCOULOMB");
//     let (q1, q2) = (entity.get_charge(), other.get_charge());
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const K: f64 = 1.; // TODO
//     let force = K * (q1 * q2) / (r * r);
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
// fn _force_newton(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
//     println!("\t    NEWTON");
//     let (m1, m2) = (entity.get_mass(), other.get_mass());
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     let u: Vec<f64> = u.iter().map(|i| i / r).collect();
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const G: f64 = 1.; // TODO
//     let force = G * (m1 * m2) / (r * r);
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
// fn _force_lennard_jones(
//     entity: &Box<dyn PhysicalObject>,
//     other: &Box<dyn PhysicalObject>,
// ) -> [f64; 3] {
//     println!("\t    LENNARD-JONES");
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const E: f64 = 1.;
//     const S: f64 = 1.;
//     let force = E * ((S / r).powf(13.) - (S / r).powf(7.));
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
// fn _force_hooke(entity: &Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) -> [f64; 3] {
//     println!("\t    HOOKE");
//     let (y1, y2) = (entity.get_position(), other.get_position());
//     let u: Vec<f64> = (0..3).map(|i| y2[i] - y1[i]).collect();
//     let r = u.iter().map(|i| i * i).sum::<f64>().powf(0.5);
//     if r == 0. {
//         return [0., 0., 0.]; // TODO sort out self in tree
//     }
//     const K: f64 = 1.; // TODO
//     let force = K * r;
//     let force: Vec<f64> = (0..3).map(|i| u[i] * force).collect();
//     let force = [force[0], force[1], force[2]];
//     force
// }
