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
        // self.context.set_font("80px");
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
    pub fn draw_line(&mut self, from: [f64; 2], to: [f64; 2]) {
        // from = self.rescale_vec(from);
        // to = self.rescale_vec(to);
        self.context.begin_path();
        self.context.move_to(from[0], from[1]);
        self.context.line_to(to[0], to[1]);
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
    pub fn draw_circle(&mut self, center: [f64; 2], radius: f64, fill: bool) {
        // let center = self.rescale_vec(center);
        let center = fix_coords(center, self.dimensions);
        // let radius = self.rescale_dist(radius);
        // let center = [center[0] * self.dimensions.0, center[1] * self.dimensions.1];

        self.context.begin_path();
        match self.context.arc(center[0], center[1], radius, 0.0, TAU) {
            Ok(_) => {
                // dom::console_log!(" + {:?}", center);
            }
            Err(_) => {
                // dom::console_log!(" - {:?}", center);
            }
        }

        self.context.stroke();
        if fill {
            self.context.fill();
        }
    }
    /// Fills Rectangle on Canvas
    pub fn fill_rect(&mut self, center: [f64; 2], width: f64, height: f64) {
        // let center = self.rescale_vec(center);
        // let width = self.rescale_dist(width);
        // let height = self.rescale_dist(height);
        self.context.begin_path();
        self.context.fill_rect(center[0], center[1], width, height)
    }
}

fn fix_coords(pos: [f64; 2], cnv_dim: (f64, f64)) -> [f64; 2] {
    // let res = [pos[0] * cnv_dim.0, pos[1] * cnv_dim.1];
    // convert from x-range [-1, 1] to x-range [0, 1] * canvas_width
    let res = [
        0.5 * (pos[0] + 1.) * cnv_dim.0,
        0.5 * (pos[1] + 1.) * cnv_dim.1,
    ];
    res
}
