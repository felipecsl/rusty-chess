extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

cfg_if! {
  // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
  // allocator.
  if #[cfg(feature = "wee_alloc")] {
    extern crate wee_alloc;
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
  }
}

#[wasm_bindgen]
extern {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
  fn alert(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
  draw_board();
}

fn draw_board() {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas_size = 320.0;
  let color_1 = "#f4d9b0";
  let color_2 = "#bc865c";
  let square_size = canvas_size / 8.0;
  let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();
  let context = canvas
      .get_context("2d")
      .unwrap()
      .unwrap()
      .dyn_into::<web_sys::CanvasRenderingContext2d>()
      .unwrap();
  context.set_line_width(2.0);
  context.stroke_rect(0.0, 0.0, canvas_size, canvas_size);
  for y in 0..8 {
    for x in 0..8 {
      let color = if (x + y) % 2 == 0 { color_1 } else { color_2 };
      context.set_fill_style(&JsValue::from(color));
      context.fill_rect(
        square_size * x as f64, square_size * y as f64, square_size, square_size);
    }
  }
}