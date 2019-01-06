extern crate wasm_bindgen;

use engine::canvas_board::wasm_bindgen::prelude::*;
use engine::piece::Piece;
use engine::board::*;
use engine::canvas_board::wasm_bindgen::JsCast;

#[allow(dead_code)]
pub struct CanvasBoard<'a> {
  pub board: &'a Board,
  pub size: f64,
  pub all_pieces: &'a Vec<&'a Piece>,
}

impl<'a> CanvasBoard<'a> {
  #[allow(dead_code)]
  pub fn print(&self) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
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
    self.print_to_canvas(&context);
  }

  fn square_size(&self) -> f64 {
    self.size / 8.0
  }
  fn piece_size(&self) -> f64 {
    self.size / 10.0
  }

  fn print_to_canvas(&self, context: &web_sys::CanvasRenderingContext2d) {
    let color_1 = "#f4d9b0";
    let color_2 = "#bc865c";
    let square_size = self.square_size();
    context.set_font(&format!("{}px Courier New", self.piece_size()));
    for y in 0..8 {
      for x in 0..8 {
        let color = if (x + y) % 2 == 0 { color_1 } else { color_2 };
        context.set_fill_style(&JsValue::from(color));
        let x_pos = square_size * x as f64;
        let y_pos = square_size * y as f64;
        context.fill_rect(x_pos, y_pos, square_size, square_size);
      }
    }
    context.set_fill_style(&JsValue::from("black"));
    for y in 0..2 {
      for x in 0..8 {
        self.draw_piece_at(x, y, context);
      }
    }
    context.set_fill_style(&JsValue::from("white"));
    for y in 6..8 {
      for x in 0..8 {
        self.draw_piece_at(x, y, context);
      }
    }
  }

  fn draw_piece_at(&self, x: u32, y: u32, context: &web_sys::CanvasRenderingContext2d) {
    let x_pos = self.square_size() * x as f64;
    let y_pos = self.square_size() * y as f64;
  let piece = self.board.piece_at(self.all_pieces, x, y);
    match context.fill_text(
      &piece_to_str(piece),
      x_pos + 7.0,
      y_pos + self.piece_size()
    ) {
      Err(_) => println!("Failed to write text"),
      Ok(_) => (),
    };
  }
}