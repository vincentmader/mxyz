use serde::{Deserialize, Serialize};

mod item;
use item::NavGridItem;

#[derive(Serialize, Deserialize)]
pub struct NavGridSection {
    section_id: String,
    title: String,
    items: Vec<NavGridItem>,
}
impl NavGridSection {
    pub fn new(section_id: &str, title: &str) -> Self {
        let items: Vec<NavGridItem> = match section_id {
            "gravity" => Vec::from([
                // intro
                // NavGridItem::new("nbody-solar", "solar system"),
                // basics
                // NavGridItem::new("2body-kepler", "Kepler's laws"),
                // NavGridItem::new("3body-moon", "Moon"),
                // NavGridItem::new("3body-lagrange", "Lagrange points"),
                // NavGridItem::new("nbody-flowers", "sym. constellations"),
                // more complex systems
                // NavGridItem::new("nbody-binary", "stellar binary"),
                // NavGridItem::new("nbody-asteroids", "asteroids"),
                // NavGridItem::new("nbody-galaxy", "galaxy"),
                // NavGridItem::new("nbody-cluster", "stellar cluster (3D)"),
                // stable 3-body
                NavGridItem::new("nbody-random", "nbody-random", "active"),
                // NavGridItem::new("3body-fig8", "figure-8", "active"),
                // NavGridItem::new("nbody-cloud", "stellar cloud", "inactive"),
                // NavGridItem::new("nbody-misc", "misc"),
                // NavGridItem::new("3body-broucke", "broucke"),
                // NavGridItem::new("3body-liao", "liao"),
                // NavGridItem::new("3body-freefall", "free-fall"),
                // NavGridItem::new("3body-moth", "moth"),
            ]),
            // "oscillators" => Vec::from([
            //     NavGridItem::new("harmonic-oscillator", "harmonic oscillator"),
            //     NavGridItem::new("single-pendulum", "single pendulum"),
            //     NavGridItem::new("double-pendulum", "double pendulum"),
            //     NavGridItem::new("lissajous", "Lissajous figures"),
            //     NavGridItem::new("fourier", "fourier"),  // ...?
            // ]),
            "electro-magnetism" => Vec::from([
                // NavGridItem::new("charge-interaction", "charge interaction"),
                // NavGridItem::new("charge-field-interaction", "charge-field interaction"),
                // NavGridItem::new("magnet", "magnet"),  // TODO rename? (multiple entries)
                // NavGridItem::new("wien-filter", "Wien filter"),
                // NavGridItem::new("field-field-interaction", "waves"),
                // NavGridItem::new("electric-motor", "electric-motor"),
            ]),
            "thermo-dynamics" => Vec::from([
                // NavGridItem::new("ising-model", "Ising model"),
                // NavGridItem::new("thermal-motion", "thermal motion"),
                // NavGridItem::new("brownian-motion", "Brownian motion"),
                // NavGridItem::new("heat-conduction", "heat conduction"),
                // NavGridItem::new("melting", "ice cubes"), // ?
            ]),
            // "fluid-dynamics" => Vec::from([
            // NavGridItem::new("diffusion", "Diffusion"),
            // NavGridItem::new("incompressible-fluid", "incompressible fluid"),
            // NavGridItem::new("raptor-flow", "raptor engine"),
            // ]),
            "quantum-mechanics" => Vec::from([
                //     NavGridItem::new("double-slit", "double slit experiment"),
                //     NavGridItem::new("feynman-graphs", "Feynman graphs"),
                // NavGridItem::new("lennard-jones", "Lennard-Jones potential"),
            ]),
            "emergent-behavior" => Vec::from([
                //     NavGridItem::new("boids", "boids"),
                //     NavGridItem::new("ants", "ants"),
                // NavGridItem::new("game-of-life", "game of life"),
            ]),
            "various" => Vec::from([
                // NavGridItem::new("rock-paper-scissors", "rock-paper-scissors"),
                    // NavGridItem::new("ca-rulemaker", "cell.aut. rule-maker"),
                // NavGridItem::new("mc-pi", "Monte Carlo pi"),
                // NavGridItem::new("hsl-colors", "HSL colors"),
            ]),
            _ => Vec::new(),
        };

        NavGridSection {
            section_id: String::from(section_id),
            title: String::from(title), // TODO -> title
            items,
        }
    }
}
