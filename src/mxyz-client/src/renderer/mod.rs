pub mod components;
use crate::renderer::components::canvas::Canvas;
use mxyz_engine::state::State;
use mxyz_engine::system::System;

const CANVAS_ID: u8 = 0; // TODO move?
const DRAWING_RADIUS: f64 = 5.;

/// Renderer
pub struct Renderer {
    pub canvas: Canvas,
}

impl Renderer {
    /// Create new Renderer.
    pub fn new() -> Self {
        let canvas = Canvas::new(CANVAS_ID);
        Renderer { canvas }
    }
    /// Initialize Renderer.
    pub fn init(&mut self) {
        self.canvas.set_fill_style("purple");
        self.canvas.set_stroke_style("purple");
    }
    /// Draw State.
    pub fn draw_state(&mut self, state: &State) {
        for system in state.systems.iter() {
            self.draw_system(system);
        }
    }
    /// Draw System.
    pub fn draw_system(&mut self, system: &System) {
        self.canvas.clear();
        for entity in system.entities.iter() {
            // match system.variant {} // TODO

            let pos = entity.get_position();
            let pos = [pos[0], pos[1]];
            self.canvas.draw_circle(pos, DRAWING_RADIUS, true);
        }
    }
}
