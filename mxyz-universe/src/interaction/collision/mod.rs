#[derive(Debug)]
pub struct Collision {
    _dampening: Option<f64>,
}
impl Collision {
    pub fn new() -> Self {
        let dampening = Some(0.); // TODO default 0 or 1?
        Collision {
            _dampening: dampening,
        }
    }
}
