use serde::{Deserialize, Serialize};

mod section;
use section::NavGridSection;

#[derive(Serialize, Deserialize)]
pub struct NavGrid {
    sections: Vec<NavGridSection>,
}
impl NavGrid {
    pub fn new() -> Self {
        let grid_section_titles: Vec<(&str, &str)> = Vec::from([
            ("newtonian-gravity", "newtonian gravity"),
            ("oscillators", "oscillators"),
            ("thermo-dynamics", "thermo-dynamics"),
            ("fluid-dynamics", "fluid dynamics"),
            ("electro-magnetism", "electro-magnetism"),
            ("emergent-behavior", "emergent behavior"),
            ("quantum-mechanics", "quantum mechanics"),
            ("various", "various"),
            ("optics", "optics"), // ?
        ]);

        let mut sections: Vec<NavGridSection> = Vec::new();
        for (id, title) in grid_section_titles.iter() {
            sections.push(NavGridSection::new(id, title));
        }

        NavGrid { sections }
    }
}
