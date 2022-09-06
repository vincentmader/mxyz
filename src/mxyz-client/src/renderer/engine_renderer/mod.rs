use crate::renderer::components::canvas::Canvas;
use crate::utils::dom;
use mxyz_engine::state::SizedState;
use mxyz_engine::state::UnsizedState;
use mxyz_engine::system::sized_system::sized_system_variant::SizedSystemVariant;
use mxyz_engine::system::sized_system::SizedSystem;
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
    pub fn draw_state(&mut self, state: &SizedState) {
        self.canvas.clear();
        dom::console_log!("aaaaaaaaaaaaa");
        for system in state.systems.iter().enumerate() {
            dom::console_log!("SUCCESS {:?}", system.0);
            self.draw_system(system);
        }
    }
    /// Draw System.
    pub fn draw_system(&mut self, system: (usize, &SizedSystem)) {
        let (system_id, system) = system;
        match &system.variant {
            SizedSystemVariant::EntitiesV1(sys) => {
                for (entity_id, entity) in sys.entities.iter().enumerate() {
                    dom::console_log!("{:?}", entity_id);
                    let pos = entity.position;
                    let pos = [pos[0], pos[1]];
                    dom::console_log!("{:?}", pos);
                    // dom::console_log!("sys-ent : {}-{}\n - pos: {:?}", system_id, entity_id, pos);
                    self.canvas.draw_circle(pos, DRAWING_RADIUS, true);
                }
            }
            _ => {}
        }
    }
}
