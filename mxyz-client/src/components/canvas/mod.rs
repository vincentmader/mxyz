use crate::utils::dom;
// use crate::utils::dom::HTMLObject;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

const TAU: f64 = 2.0 * std::f64::consts::PI;

/// Canvas
pub struct Canvas {
    // pub context: web_sys::CanvasRenderingContext2d,
    id: Option<u8>,
    dimensions: (f64, f64),

    element: HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    scale: f64,
    pub zoom: f64,
}
// impl HTMLObject for Canvas {
//     fn update(&mut self) {
//         let from = (0., 0.);
//         let to = (1000., 1000.);
//         self.set_stroke_style("white");
//         self.draw_line(from, to);
//     }
// }
impl Canvas {
    pub fn new(id: u8) -> Self {
        let canvas_id = format!("canvas_{}", id);

        let id = Option::Some(id);
        let dimensions = (0., 0.); // TODO

        let element = dom::document()
            .get_element_by_id(&canvas_id)
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let scale = 1.;
        let canvas_width = scale * f64::from(element.width());
        let canvas_height = scale * f64::from(element.height());
        let dimensions = (canvas_width, canvas_height);
        // let zoom = match centered { true => 0.5, _ => 1.}; // TODO make changeable
        let zoom = 0.5;

        Canvas {
            id,
            dimensions,
            zoom,
            scale,
            element,
            context,
        }
    }
    pub fn clear(&mut self) {
        let w = self.dimensions.0;
        let h = self.dimensions.1;
        self.context.clear_rect(0., 0., w, h);
    }
    pub fn set_font(&mut self, font: &str) {
        self.context.set_font(&font);
    }
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        self.context.fill_text(&text, x, y).unwrap();
    }
    pub fn reset_line_width(&mut self) {
        self.context.set_line_width(1.);
    }
    pub fn set_line_width(&mut self, width: f64) {
        self.context.set_line_width(width);
    }
    pub fn set_stroke_style(&mut self, color: &str) {
        self.context.set_stroke_style(&JsValue::from_str(&color));
    }
    pub fn set_fill_style(&mut self, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(&color));
    }
    pub fn rescale_vec(&self, vec: (f64, f64)) -> (f64, f64) {
        // re-scale to canvas dimensions [px]   // TODO include scale
        let mut vec = vec;
        vec.0 *= self.zoom * self.dimensions.0 / self.scale;
        vec.1 *= self.zoom * self.dimensions.1 / self.scale;
        // center
        vec.0 += 0.5 * self.dimensions.0;
        vec.1 += 0.5 * self.dimensions.1;
        vec
    }
    pub fn rescale_dist(&self, dist: f64) -> f64 {
        dist * self.dimensions.1 * self.zoom / self.scale // only works for square
    }
    pub fn draw_line(&mut self, mut from: (f64, f64), mut to: (f64, f64)) {
        from = self.rescale_vec(from);
        to = self.rescale_vec(to);
        self.context.begin_path();
        self.context.move_to(from.0, from.1);
        self.context.line_to(to.0, to.1);
        self.context.stroke();
    }
    pub fn draw_triangle(
        &mut self,
        mut first: (f64, f64),
        mut second: (f64, f64),
        mut third: (f64, f64),
    ) {
        first = self.rescale_vec(first);
        second = self.rescale_vec(second);
        third = self.rescale_vec(third);
        self.context.begin_path();
        self.context.move_to(first.0, first.1);
        self.context.line_to(second.0, second.1);
        self.context.line_to(third.0, third.1);
        self.context.stroke();
        self.context.fill()
    }
    pub fn draw_circle(&mut self, center: (f64, f64), radius: f64, fill: bool) {
        let center = self.rescale_vec(center);
        let radius = self.rescale_dist(radius);
        self.context.begin_path();
        self.context
            .arc(center.0, center.1, radius, 0.0, TAU)
            .unwrap();
        self.context.stroke();
        if fill {
            self.context.fill();
        }
    }
    pub fn fill_rect(&mut self, center: (f64, f64), width: f64, height: f64) {
        let center = self.rescale_vec(center);
        let width = self.rescale_dist(width);
        let height = self.rescale_dist(height);
        self.context.begin_path();
        self.context.fill_rect(center.0, center.1, width, height)
    }
}
