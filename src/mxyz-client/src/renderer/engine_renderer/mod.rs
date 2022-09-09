use crate::renderer::components::canvas::Canvas;
use crate::utils::dom;
use mxyz_engine::config::engine_runner_variant::EngineRunnerVariant;
use mxyz_engine::config::EngineConfig;
use mxyz_engine::state::UnsizedState;
use mxyz_engine::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
use mxyz_engine::system::unsized_system::UnsizedSystem;

const CANVAS_ID: u8 = 0; // TODO move?
const DRAWING_RADIUS: f64 = 3.;

/// Renderer
pub struct EngineRenderer {
    pub canvas: Canvas,
    engine_runner_variant: EngineRunnerVariant,
}

impl EngineRenderer {
    /// Create new Renderer.
    pub fn new(engine_runner_variant: EngineRunnerVariant) -> Self {
        let canvas = Canvas::new(CANVAS_ID);
        EngineRenderer {
            engine_runner_variant,
            canvas,
        }
    }
    /// Initialize Renderer.
    pub fn init(&mut self) {
        self.canvas.set_fill_style("white");
        self.canvas.set_stroke_style("white");
    }
    /// Draw State.
    pub fn display_state(&mut self, state: &UnsizedState, config: Option<&mut EngineConfig>) {
        self.canvas.clear();
        self.display_interaction_matrix();
        for system in state.systems.iter().enumerate() {
            self.draw_system(&system);
            let update_all = true; // TODO only on change in menus (?)
            if state.state_id == 0 || update_all {
                self.display_system(&system)
            }
        }
    }
    pub fn display_interaction_matrix(&mut self) {
        // let interactions =
        // for (interaction_idx, interaction) in interactions.iter().enumerate() {}
    }
    pub fn display_system(&mut self, system: &(usize, &UnsizedSystem)) {
        let (system_id, system) = system;

        // HTML Section Element
        let section_element_id = format!("system-section-{}", system_id);
        let section = get_or_create_element("page-column-right", "div", &section_element_id);
        section.set_class_name("section");

        // HTML Header4 Element, Section Title
        let title_element_id = format!("system-section-{}-title", system_id);
        let title = get_or_create_element(&section_element_id, "h4", &title_element_id);
        title.set_text_content(Some(&format!(
            "System {}, entities: {}",
            system_id,
            system.entities.len()
        )));

        // HTML p Element, Integrators Title
        let integrators_title_el_id = format!("system-section-{}-integrators-title", system_id);
        let title = get_or_create_element(&section_element_id, "p", &integrators_title_el_id);
        title.set_text_content(Some(&format!(
            "Integrators: ({})",
            system.integrators.len()
        )));

        //
        let integrators_id = format!("system-section-{}-integrators", system_id);
        let integrators = get_or_create_element(&section_element_id, "ul", &integrators_id);

        for (integrator_idx, integrator) in system.integrators.iter().enumerate() {
            //
            let integrator_id =
                format!("system-section-{}-integrator-{}", system_id, integrator_idx);
            let integrator_el = get_or_create_element(&integrators_id, "li", &integrator_id);
            let integrator_title: String = (&integrator.variant).to_string();
            integrator_el.set_inner_html(&format!("<p>{}</p><ul></ul>", integrator_title));

            //
            let interactions_id = format!(
                "system-section-{}-integrator-{}-interactions",
                system_id, integrator_idx
            );
            let interactions_el = get_or_create_element(&integrator_id, "ul", &interactions_id);
            // let integrator_title: String = (&integrator.variant).to_string();
            // integrator_el.set_inner_html(&format!("<p>{}</p><ul></ul>", integrator_title));

            for (interaction_idx, interaction) in integrator.interactions.iter().enumerate() {
                //
                let interaction_id = format!(
                    "system-section-{}-integrator-{}-interaction-{}",
                    system_id, integrator_idx, interaction_idx
                );
                let interaction_el = get_or_create_element(&interactions_id, "li", &interaction_id);
                let interaction_title: String = (&interaction.variant).to_string();
                interaction_el.set_text_content(Some(&interaction_title));
            }
        }

        // let foo = "aaa";
        // html_title.set_text_content(Some(foo));
        // html_section.append_child(&html_title).unwrap();

        // let foo_str_integrators = format!(
        //     "<p>Integrators ({})</p>
        //      <ul>
        //        <li>...</li>
        //      </ul>",
        //     system.integrators.len(),
        // );
        // let entities = &system.entities;
        // let system_size = entities.len();

        // html_section.set_inner_html(&format!(
        //     "<div>
        //        <p>System {} ({} entities)</p>
        //        <ul>
        //          <li>html-id {}</li>
        //        </ul>
        //        {}
        //      </div>",
        //     system_id, system_size, section_element_id, foo_str_integrators
        // ));

        // let foo = dom::document().create_element("div").unwrap();
        // foo.set_i

        // html_section.append_child(&foo).unwrap();
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

fn get_or_create_element(parent_id: &str, element: &str, element_id: &str) -> web_sys::Element {
    match dom::document().get_element_by_id(&element_id) {
        Some(element) => element,
        None => {
            let parent = dom::document().get_element_by_id(parent_id).unwrap();
            let element = dom::document().create_element(element).unwrap();
            element.set_id(element_id);
            parent.append_child(&element).unwrap();
            element
        }
    }
}
