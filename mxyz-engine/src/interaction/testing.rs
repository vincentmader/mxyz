// use crate::entity::field::DiscreteFieldCell;
// use crate::entity::object::PhysicalObject;
use mxyz_universe::entity::Entity as PhysicalObject;

fn _coulomb(entity: &mut Box<dyn PhysicalObject>, other: &Box<dyn PhysicalObject>) {
    // parameters
    let k = 1.;
    let dt = 0.001;
    // info from entities for force-calculation
    let m1 = entity.get_mass();
    let q1 = entity.get_charge();
    let x1 = entity.get_position();
    let q2 = other.get_charge();
    let x2 = other.get_position();
    // intermediate steps
    let u = [x2[0] - x1[0], x2[1] - x1[1], x2[2] - x1[2]];
    let r = u.iter().map(|u| u * u).sum::<f64>().powf(0.5);
    // force-calculation
    let force = k * (q1 * q2) / (r * r);
    let force = [force * u[0], force * u[1], force * u[2]];
    // velocity updating (e.g. explicit, Euler)
    let v1 = entity.get_velocity();
    let dv = [force[0] / m1 * dt, force[1] / m1 * dt, force[2] / m1 * dt];
    let v1 = [v1[0] + dv[0], v1[1] + dv[1], v1[2] + dv[1]];
    entity.set_velocity(&v1);
}
