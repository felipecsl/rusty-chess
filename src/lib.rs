extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

use cfg_if::cfg_if;

use engine::board::Board;
use engine::canvas_board::CanvasBoardRenderer;
use engine::game_controller::GameController;

use self::wasm_bindgen::prelude::*;

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
  let controller = GameController::new();
  let board = Box::new(Board::new());
  // We need the Board object to live for the entire duration of the program.
  // Thus, we leak it to obtain a 'static reference
  let board_ref = Box::leak(board);
  let renderer = CanvasBoardRenderer::new(board_ref);
  renderer.render(controller.context);
  controller.bind_event_handlers(board_ref, renderer);
}