// use super::components::canvas::Canvas;
// use super::utils::dom;
// use super::utils::dom::HTMLObject;
// use crate::utils::dom::console_log;
// use mxyz_network::package::Package;
// use std::sync::mpsc::Receiver;
use mxyz_config::SimulationVariant;

// const canvas_id: u8 = 0; // TODO

/// Renderer
pub struct Renderer {
    // rx_web_to_render: Receiver<Package>,
// pub canvases: Vec<Canvas>,
// entities: Vec<Box<dyn HTMLObject>>
}

impl Renderer {
    /// Create new Renderer
    pub fn new(
        // rx_web_to_render: Receiver<Package>
        _simulation_variant: &SimulationVariant,
    ) -> Self {
        // let canvases = vec![];
        // let entities = vec![];
        Renderer {
            // rx_web_to_render,
            // canvases,
            // entities
        }
    }

    /// Initialize Renderer
    // pub fn init(&mut self, rx_web_to_render: Receiver<Package>) {
    pub fn init(&mut self) {
        // match self.simulation_variant {
        //     _ => {}
        // }
        // loop {
        // let a = self.rx_web_to_render.recv().unwrap();
        // let a = rx_web_to_render.recv().unwrap();
        // console_log(&format!("{:?}", a));
        // }
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
