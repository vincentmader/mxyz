use mxyz_universe::entity::Entity;

// TODO move else-where
const G: f64 = 1.; // Newton Gravity

pub fn from(entity: &Box<dyn Entity>, other: &Box<dyn Entity>) -> [f64; 3] {
    let (x1, x2) = (entity.get_position(), other.get_position());
    let rel_x = [x2[0] - x1[0], x2[1] - x1[1], x2[2] - x1[2]];
    let dist = rel_x.iter().map(|dx| dx * dx).sum::<f64>().powf(0.5);
    let unit = [rel_x[0] / dist, rel_x[1] / dist, rel_x[2] / dist];
    let force = todo!();
    force
}
