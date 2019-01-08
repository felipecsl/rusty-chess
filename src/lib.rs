extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

mod engine;
mod utils;

use self::wasm_bindgen::prelude::*;
use self::wasm_bindgen::JsCast;
use self::web_sys::CanvasRenderingContext2d;
use self::web_sys::Document;
use self::web_sys::HtmlCanvasElement;
use self::web_sys::MouseEvent;
use cfg_if::cfg_if;
use engine::board::Board;
use engine::canvas_board::CanvasBoardRenderer;
use std::rc::Rc;

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
  let document = get_document();
  let canvas = get_canvas(&document);
  let context = init_context(&canvas);
  let board = Rc::new(engine::board::new_board());
  let canvas_board_renderer = CanvasBoardRenderer {
    board: board.clone(),
  };
  let click_handler = new_onclick_handler(board);
  bind_click_handler(&canvas, click_handler);
  canvas_board_renderer.render(&context);
}

fn new_onclick_handler(board: Rc<Board>) -> Box<dyn Fn(MouseEvent)> {
  Box::new(move |event: MouseEvent| {
    let x = event.offset_x() as u32 / SQUARE_SIZE as u32;
    let y = event.offset_y() as u32 / SQUARE_SIZE as u32;
    let piece = board.piece_at(x, y);
    match piece {
      Some(p) => log(&format!("Clicked piece {:?}", p)),
      None => log("no piece on this position"),
    };
  })
}

fn get_document() -> Document {
  web_sys::window().unwrap().document().unwrap()
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

fn bind_click_handler(canvas: &HtmlCanvasElement, func: Box<dyn Fn(MouseEvent)>) {
  let closure = Closure::wrap(func);
  let res = canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
  match res {
    Ok(_) => (),
    Err(_) => panic!("Failed to add click event listener"),
  }
  closure.forget();
}
