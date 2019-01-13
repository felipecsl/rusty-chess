extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

use cfg_if::cfg_if;

use engine::canvas_board::CanvasBoardRenderer;
use engine::game_controller::GameController;

use self::wasm_bindgen::prelude::*;
use self::wasm_bindgen::JsCast;
use self::web_sys::CanvasRenderingContext2d;
use self::web_sys::Document;
use self::web_sys::HtmlCanvasElement;
use self::web_sys::MouseEvent;

mod engine;
mod utils;

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
extern "C" {
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

static CANVAS_SIZE: f64 = 600.0;
static SQUARE_SIZE: f64 = CANVAS_SIZE / 8.0;
static PIECE_SIZE: f64 = CANVAS_SIZE / 10.0;

#[wasm_bindgen(start)]
pub fn start() {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = get_canvas(&document);
  let context = init_context(&canvas);
  let renderer = Box::new(CanvasBoardRenderer::new(context));
  let renderer_ref = Box::leak(renderer);
  let controller = GameController::new(renderer_ref);
  controller.render();
  let func = new_onclick_event(controller);
  let closure = Closure::wrap(func);
  add_click_handler(&canvas, closure);
}

fn new_onclick_event(mut controller: GameController<'static>) -> Box<FnMut(MouseEvent)> {
  Box::new(move |event: MouseEvent| {
    let x = event.offset_x() as u32 / SQUARE_SIZE as u32;
    let y = event.offset_y() as u32 / SQUARE_SIZE as u32;
    controller.handle_click_at(x, y);
  }) as Box<FnMut(MouseEvent)>
}

fn add_click_handler(canvas: &HtmlCanvasElement, closure: Closure<dyn FnMut(MouseEvent)>) {
  if let Err(_) = canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
  {
    panic!("Failed to add click event listener");
  }
  closure.forget();
}

fn get_canvas(document: &Document) -> HtmlCanvasElement {
  let element = document.get_element_by_id("canvas").unwrap();
  element
    .dyn_into::<HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap()
}

fn init_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
  canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<CanvasRenderingContext2d>()
    .unwrap()
}
