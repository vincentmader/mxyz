use super::renderer::components::canvas::Canvas;

pub fn draw(i: usize) {
    let r = i as f64 * 0.3;
    let phi = i as f64 * 0.1;
    let pos = (r * phi.cos(), r * phi.sin());
    let pos = (pos.0 + 500., pos.1 + 500.);

    let mut canvas = Canvas::new(0);
    canvas.set_stroke_style("blue");
    canvas.set_fill_style("blue");
    canvas.draw_circle(pos, 4., true);
}
