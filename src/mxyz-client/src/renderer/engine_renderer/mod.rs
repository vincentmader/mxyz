use crate::renderer::components::canvas::Canvas;
use crate::utils::dom;
use mxyz_engine::state::UnsizedState;
use mxyz_engine::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
use mxyz_engine::system::unsized_system::UnsizedSystem;

const CANVAS_ID: u8 = 0; // TODO move?
const DRAWING_RADIUS: f64 = 3.;

/// Renderer
pub struct EngineRenderer {
    pub canvas: Canvas,
}

impl EngineRenderer {
    /// Create new Renderer.
    pub fn new() -> Self {
        let canvas = Canvas::new(CANVAS_ID);
        EngineRenderer { canvas }
    }
    /// Initialize Renderer.
    pub fn init(&mut self) {
        self.canvas.set_fill_style("white");
        self.canvas.set_stroke_style("white");
    }
    /// Draw State.
    pub fn draw_state(&mut self, state: &UnsizedState) {
        self.canvas.clear();
        // dom::console_log!("\nstate {}", state.state_id);
        for system in state.systems.iter().enumerate() {
            self.draw_system(system);
        }
    }
    /// Draw System.
    pub fn draw_system(&mut self, system: (usize, &UnsizedSystem)) {
        let (system_id, system) = system;
        for (entity_id, entity) in system.entities.iter().enumerate() {
            match system.variant {
                // SystemVariant::EntitiesV1 => {}
                _ => {
                    let pos = entity.get_position();
                    let pos = [pos[0], pos[1]];
                    // dom::console_log!("sys-ent : {}-{}\n - pos: {:?}", system_id, entity_id, pos);

                    self.canvas.draw_circle(pos, DRAWING_RADIUS, true);
                }
            }
        }
    }
}
