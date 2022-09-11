use crate::entity::Entity;

// TODO move else-where
const SIGMA: f64 = 0.01; // Newton Gravity
const EPSILON: f64 = 10.; // Newton Gravity

pub fn from(
    entity: &Box<dyn Entity>,
    other: &Box<dyn Entity>,
    entities: &Vec<Box<dyn Entity>>,
) -> [f64; 3] {
    let (x1, x2) = (entity.get_position(), other.get_position());
    let rel_x = [x2[0] - x1[0], x2[1] - x1[1], x2[2] - x1[2]];
    let r = rel_x.iter().map(|dx| dx * dx).sum::<f64>().powf(0.5);
    let unit = [rel_x[0] / r, rel_x[1] / r, rel_x[2] / r];
    let force = 4. * EPSILON * (12. * (SIGMA / r).powf(13.) - 6. * (SIGMA / r).powf(5.));
    let force = [force * unit[0], force * unit[1], force * unit[2]];
    force
}
