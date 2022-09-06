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
            "newtonian-gravity" => Vec::from([
                // intro
                NavGridItem::new("newtonian-gravity", "solar-system", "solar system", 0),
                // basics
                NavGridItem::new("newtonian-gravity", "2body-kepler", "laws of kepler", 0),
                NavGridItem::new("newtonian-gravity", "3body-moon", "moons", 1),
                NavGridItem::new("newtonian-gravity", "3body-lagrange", "lagrange points", 0),
                NavGridItem::new(
                    "newtonian-gravity",
                    "symmetric-satellites",
                    "sym. satellite const.",
                    0,
                ),
                // more complex systems
                NavGridItem::new(
                    "newtonian-gravity",
                    "nbody-binary",
                    "binary star systems",
                    0,
                ),
                // NavGridItem::new("newtonian-gravity", "nbody-asteroids", "asteroids", 0),
                // NavGridItem::new("newtonian-gravity", "nbody-galaxy", "galaxy", 0),
                NavGridItem::new(
                    "newtonian-gravity",
                    "nbody-cluster",
                    "stellar cluster (3d)",
                    0,
                ),
                // stable 3-body
                // NavGridItem::new("newtonian-gravity", "nbody-random", "nbody-random", 0),
                NavGridItem::new("newtonian-gravity", "3body-fig8", "figure-8 orbit", 0),
                // NavGridItem::new("newtonian-gravity", "nbody-cloud", "stellar cloud", 0),
                // NavGridItem::new("newtonian-gravity", "nbody-misc", "misc", 0),
                // NavGridItem::new("newtonian-gravity", "3body-broucke", "broucke", 0),
                // NavGridItem::new("newtonian-gravity", "3body-liao", "liao", 0),
                // NavGridItem::new("newtonian-gravity", "3body-freefall", "free-fall", 0),
                // NavGridItem::new("newtonian-gravity", "3body-moth", "moth", 0),
            ]),
            "oscillators" => Vec::from([
                NavGridItem::new(
                    "oscillators",
                    "harmonic-oscillator",
                    "harmonic oscillator",
                    0,
                ),
                NavGridItem::new("oscillators", "single-pendulum", "single pendulum", 1),
                NavGridItem::new("oscillators", "double-pendulum", "double pendulum", 1),
                NavGridItem::new("oscillators", "lissajous-figures", "lissajous figures", 1),
                // NavGridItem::new("oscillators", "fourier", "fourier", 0), // ...?
            ]),
            "electro-magnetism" => Vec::from([
                NavGridItem::new(
                    "electro-magnetism",
                    "nbody-charge-interaction",
                    "charge interaction",
                    0,
                ),
                NavGridItem::new(
                    "gravity",
                    "charge-field-interaction",
                    "charge-field interaction",
                    0,
                ),
                NavGridItem::new("electro-magnetism", "magnet", "magnet", 0), // TODO rename? (multiple entries)
                NavGridItem::new("electro-magnetism", "wien-filter", "Wien filter", 1),
                NavGridItem::new("electro-magnetism", "field-field-interaction", "waves", 0),
                // NavGridItem::new("electro-magnetism", "electric-motor", "electric-motor", 0),
            ]),
            "thermo-dynamics" => Vec::from([
                NavGridItem::new("thermo-dynamics", "ising-model", "ising model", 0),
                NavGridItem::new("thermo-dynamics", "thermal-motion", "thermal motion", 1),
                NavGridItem::new("thermo-dynamics", "brownian-motion", "brownian motion", 1),
                NavGridItem::new("thermo-dynamics", "heat-conduction", "heat conduction", 0),
                // NavGridItem::new("thermo-dynamics", "melting", "ice cubes", 0), // ?
            ]),
            "fluid-dynamics" => Vec::from([
                NavGridItem::new("fluid-dynamics", "diffusion", "Diffusion", 0),
                NavGridItem::new(
                    "fluid-dynamics",
                    "incompressible-fluid",
                    "incompressible fluid",
                    0,
                ),
                // NavGridItem::new("fluid-dynamics", "raptor-flow", "raptor engine", 0),
            ]),
            "quantum-mechanics" => Vec::from([
                NavGridItem::new(
                    "quantum-mechanics",
                    "double-slit",
                    "double slit experiment",
                    0,
                ),
                // NavGridItem::new("quantum-mechanics", "feynman-graphs", "Feynman graphs", 0),
                NavGridItem::new(
                    "quantum-mechanics",
                    "lennard-jones",
                    "lennard-jones interaction",
                    0,
                ),
            ]),
            "emergent-behavior" => Vec::from([
                NavGridItem::new("emergent-behavior", "nbody-boids", "boids", 1),
                NavGridItem::new("emergent-behavior", "nbody-ants", "ants", 0),
                NavGridItem::new("emergent-behavior", "game-of-life", "game of life", 0),
            ]),
            "various" => Vec::from([
                NavGridItem::new("various", "rock-paper-scissors", "rock-paper-scissors", 0),
                // NavGridItem::new("various", "ca-rulemaker", "cell.aut. rule-maker", 0),
                NavGridItem::new("various", "pi-calculation", "Monte Carlo pi", 0),
                // NavGridItem::new("various", "hsl-colors", "HSL colors", 0),
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
