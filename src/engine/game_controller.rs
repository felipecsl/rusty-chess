extern crate wasm_bindgen;
extern crate web_sys;

use engine::board::Board;
use engine::canvas_board::CanvasBoardRenderer;

use self::wasm_bindgen::JsCast;
use self::wasm_bindgen::prelude::*;
use self::web_sys::CanvasRenderingContext2d;
use self::web_sys::Document;
use self::web_sys::HtmlCanvasElement;
use self::web_sys::MouseEvent;

use SQUARE_SIZE;
use log;

pub struct GameController {
  canvas: HtmlCanvasElement,
  pub context: &'static CanvasRenderingContext2d,
}

impl GameController {
  pub fn new() -> GameController {
    let document = GameController::get_document();
    let canvas = GameController::get_canvas(&document);
    let context = Box::new(GameController::init_context(&canvas));
    GameController { canvas, context: Box::leak(context), }
  }

  pub fn bind_event_handlers(&self, board: &'static Board, renderer: CanvasBoardRenderer<'static>) {
    let click_handler = GameController::new_onclick_handler(board, self.context, renderer);
    self.bind_click_handler(click_handler);
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

  fn new_onclick_handler<'a>(
    board: &'static Board<'a>,
    context: &'static CanvasRenderingContext2d,
    mut renderer: CanvasBoardRenderer<'a>,
  ) -> Box<dyn FnMut(web_sys::MouseEvent)> {
    Box::new(move |event: MouseEvent| {
      let x = event.offset_x() as u32 / SQUARE_SIZE as u32;
      let y = event.offset_y() as u32 / SQUARE_SIZE as u32;
      let piece = board.piece_at(x, y);
      match piece {
        Some(p) => renderer.select_piece(&p),
        None => log("no piece on this position"),
      };
      renderer.render(&context);
    })
  }

  fn bind_click_handler(&self, func: Box<dyn FnMut(MouseEvent)>) {
    let closure = Closure::wrap(func);
    let res = self.canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
    match res {
      Ok(_) => (),
      Err(_) => panic!("Failed to add click event listener"),
    }
    closure.forget();
  }
}