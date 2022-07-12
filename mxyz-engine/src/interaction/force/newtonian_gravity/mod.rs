use mxyz_universe::entity::Entity;

// TODO move else-where
const G: f64 = 1.; // Newton Gravity

pub fn calculate_from(entity: &Box<dyn Entity>, other: &Box<dyn Entity>) -> [f64; 3] {
    let (m1, m2) = (entity.get_mass(), other.get_mass());
    let (x1, x2) = (entity.get_position(), other.get_position());
    let rel_x = [x2[0] - x1[0], x2[1] - x1[1], x2[2] - x1[2]];
    let dist = rel_x.iter().map(|dx| dx * dx).sum::<f64>();
    let unit = [rel_x[0] / dist, rel_x[1] / dist, rel_x[1] / dist];
    let force = G * (m1 * m2) / dist.powf(2.);
    let force = [unit[0] * force, unit[1] * force, unit[2] * force];
    force
}

// for coulomb:
// -----------
// const K: f64 = 1.; // Coulomb
//
//     let (q1, q2) = (entity.get_charge(), other.get_charge());
//     let (x1, x2) = (entity.get_position(), other.get_position());
//     let rel_x = [x2[0] - x1[0], x2[1] - x1[1], x2[2] - x1[2]];
//     let dist = rel_x.iter().map(|dx| dx * dx).sum::<f64>();
//     let unit = [rel_x[0] / dist, rel_x[1] / dist, rel_x[1] / dist];
//     let force = K * (q1 * q2) / dist.powf(2.);
//     let force = [unit[0] * force, unit[1] * force, unit[2] * force];
//     force
