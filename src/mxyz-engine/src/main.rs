// #![allow(unused_doc_comments)]
use mxyz_engine::config::ExportVariant;
use mxyz_engine::Engine;
use mxyz_engine_universe::preset::SimulationVariant;

const EXPORT_VARIANT: ExportVariant = ExportVariant::ToDatabase;
const ENGINE_ID: usize = 0;

fn main() {
    // Create new engine.
    let mut engine = Engine::new(ENGINE_ID); // TODO
                                             // Initialize engine's state-vector.
    engine.init(Some(SimulationVariant::ThreeBodyMoon));
    // Initialize engine-config.
    engine.config.export_variant = EXPORT_VARIANT;

    // Run engine & record execution time.
    let start_time = std::time::Instant::now();
    {
        engine.run();
    }
    let duration = start_time.elapsed().as_millis();
    // Print information about execution time.
    println!("\nRuntime:   {} ms\t ({} h)", duration, duration / 3600000);
}
