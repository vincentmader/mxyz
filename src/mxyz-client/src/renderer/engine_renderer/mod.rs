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
    pub fn display_state(&mut self, state: &UnsizedState) {
        self.canvas.clear();
        for system in state.systems.iter().enumerate() {
            self.draw_system(&system);
            self.display_system(&system)
        }
    }
    pub fn display_system(&mut self, system: &(usize, &UnsizedSystem)) {
        let (system_id, system) = system;

        let element_id = format!("system-section-{}", system_id); // TODO
        let element = match dom::document().get_element_by_id(&element_id) {
            Some(element) => element,
            None => create_element((*system_id, system), &element_id),
        };

        fn create_element(system: (usize, &UnsizedSystem), element_id: &str) -> web_sys::Element {
            let (system_id, system) = system;

            let page_col_right = dom::document()
                .get_element_by_id("page-column-right")
                .unwrap();
            let element = dom::document().create_element("div").unwrap();
            element.set_id(element_id);
            element.set_class_name("section");

            let entities = &system.entities;
            let system_size = entities.len();
            element.set_inner_html(&format!(
                "<div>
                   <p>System {}</p>
                   <ul>
                     <li>Size {}</li>
                     <li>html-id {}</li>
                   </ul>
                   <p>Integrators</p>
                   <ul>
                     <li>...</li>
                   </ul>
                 </div>",
                system_id,
                system_size,
                element_id,
                // config.systems.get(system_id).unwrap().integrators.len(),
            ));

            page_col_right.append_child(&element).unwrap();
            element
        }

        let foo = dom::document().create_element("div").unwrap();
        // foo.set_i

        element.append_child(&foo).unwrap();
    }
    /// Draw System.
    pub fn draw_system(&mut self, system: &(usize, &UnsizedSystem)) {
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
