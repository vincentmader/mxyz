// use super::components::canvas::Canvas;
// use super::utils::dom;
// use super::utils::dom::HTMLObject;

const canvas_id: u8 = 0; // TODO

/// Renderer
pub struct Renderer {
    // pub canvases: Vec<Canvas>,
// entities: Vec<Box<dyn HTMLObject>>,
}

impl Renderer {
    /// Create new Renderer
    pub fn new() -> Self {
        // let canvases = vec![];
        // let entities = vec![];
        Renderer {
            // canvases,
            // entities
        }
    }

    /// Initialize Renderer
    pub fn init(&mut self) {
        // let cnv = Canvas::new(canvas_id);
        // self.canvases.push(cnv);
        // self.entities.push(Box::new(cnv));
    }

    /// Draw Components
    pub fn draw(&mut self) {
        // let _: Vec<()> = self.entities.iter_mut().map(|e| e.update()).collect();
        self.test();
    }

    fn test(&self) {
        // for system in current_state.systems {
        //     match system {}
        // }
    }
}
