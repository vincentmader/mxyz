use crate::utils::dom;
// use crate::utils::dom::HTMLObject;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

const TAU: f64 = 2.0 * std::f64::consts::PI;

/// Canvas
pub struct Canvas {
    /// Canvas ID: 0-255
    pub id: Option<u8>,
    /// Canvas Dimensions (TODO in pixels?)
    pub dimensions: (f64, f64),
    /// DOM HTML-Element & Rendering Context
    pub element: HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
    /// Scale & Zoom
    pub scale: f64, // TODO
    pub zoom: f64, // TODO
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
    /// Creates new Canvas Instance from Canvas-ID
    pub fn new(id: u8) -> Self {
        let id_str = format!("canvas_{}", id);
        let element = dom::document()
            .get_element_by_id(&id_str)
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
        let zoom = 0.5; // = match centered { true => 0.5, _ => 1.}; // TODO make changeable
        let canvas_width = scale * f64::from(element.width());
        let canvas_height = scale * f64::from(element.height());

        Canvas {
            id: Some(id),
            dimensions: (canvas_width, canvas_height),
            zoom,
            scale,
            element,
            context,
        }
    }

    /// Initializes Canvas
    pub fn init(&mut self) {
        self.set_stroke_style("white");
        self.set_fill_style("white");
    }

    /// Clears Canvas
    pub fn clear(&mut self) {
        let w = self.dimensions.0;
        let h = self.dimensions.1;
        self.context.clear_rect(0., 0., w, h);
    }
    /// Specifies Font
    pub fn set_font(&mut self, font: &str) {
        self.context.set_font(&font);
    }
    /// Specifies Stroke-Style
    pub fn set_stroke_style(&mut self, color: &str) {
        self.context.set_stroke_style(&JsValue::from_str(&color));
    }
    /// Specifies Fill-Style
    pub fn set_fill_style(&mut self, color: &str) {
        self.context.set_fill_style(&JsValue::from_str(&color));
    }
    /// Specifies Width of Lines
    pub fn set_line_width(&mut self, width: f64) {
        self.context.set_line_width(width);
    }
    /// Writes Text to Canvas
    pub fn fill_text(&mut self, text: &str, x: f64, y: f64) {
        self.context.fill_text(&text, x, y).unwrap();
    }
    // pub fn reset_line_width(&mut self) {
    //     self.context.set_line_width(1.);
    // }
    // pub fn rescale_vec(&self, vec: (f64, f64)) -> (f64, f64) {
    //     // re-scale to canvas dimensions [px]   // TODO include scale
    //     let mut vec = vec;
    //     vec.0 *= self.zoom * self.dimensions.0 / self.scale;
    //     vec.1 *= self.zoom * self.dimensions.1 / self.scale;
    //     // center
    //     vec.0 += 0.5 * self.dimensions.0;
    //     vec.1 += 0.5 * self.dimensions.1;
    //     vec
    // }
    // pub fn rescale_dist(&self, dist: f64) -> f64 {
    //     dist * self.dimensions.1 * self.zoom / self.scale // only works for square
    // }
    /// Draws Line on Canvas
    pub fn draw_line(&mut self, from: (f64, f64), to: (f64, f64)) {
        // from = self.rescale_vec(from);
        // to = self.rescale_vec(to);
        self.context.begin_path();
        self.context.move_to(from.0, from.1);
        self.context.line_to(to.0, to.1);
        self.context.stroke();
    }
    // pub fn draw_triangle(
    //     &mut self,
    //     first: (f64, f64),
    //     second: (f64, f64),
    //     third: (f64, f64),
    // ) {
    //     // first = self.rescale_vec(first);
    //     // second = self.rescale_vec(second);
    //     // third = self.rescale_vec(third);
    //     self.context.begin_path();
    //     self.context.move_to(first.0, first.1);
    //     self.context.line_to(second.0, second.1);
    //     self.context.line_to(third.0, third.1);
    //     self.context.stroke();
    //     self.context.fill()
    // }
    /// Draws Circle on Canvas
    pub fn draw_circle(&mut self, center: (f64, f64), radius: f64, fill: bool) {
        // let center = self.rescale_vec(center);
        // let radius = self.rescale_dist(radius);
        self.context.begin_path();
        self.context
            .arc(center.0, center.1, radius, 0.0, TAU)
            .unwrap();
        self.context.stroke();
        if fill {
            self.context.fill();
        }
    }
    /// Fills Rectangle on Canvas
    pub fn fill_rect(&mut self, center: (f64, f64), width: f64, height: f64) {
        // let center = self.rescale_vec(center);
        // let width = self.rescale_dist(width);
        // let height = self.rescale_dist(height);
        self.context.begin_path();
        self.context.fill_rect(center.0, center.1, width, height)
    }
}
