extern crate wasm_bindgen;
extern crate web_sys;

use self::wasm_bindgen::prelude::*;
use self::web_sys::CanvasRenderingContext2d;
use engine::board::*;
use PIECE_SIZE;
use SQUARE_SIZE;
use engine::piece::Piece;
use log;

#[allow(dead_code)]
pub struct CanvasBoardRenderer<'a> {
  pub board: &'a Board<'a>,
  selected_piece: Option<&'a Piece<'a>>,
}

impl<'a> CanvasBoardRenderer<'a> {
  pub fn new(board: &'a Board<'a>) -> CanvasBoardRenderer<'a> {
    CanvasBoardRenderer { board, selected_piece: None }
  }

  #[allow(dead_code)]
  pub fn render(&self, context: &CanvasRenderingContext2d) {
    let color_1 = "#f4d9b0";
    let color_2 = "#bc865c";
    let square_size = SQUARE_SIZE;
    context.set_font(&format!("{}px Courier New", PIECE_SIZE));
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

  pub fn select_piece(&mut self, piece: &'a Piece<'a>) {
    let valid_moves = piece.valid_moves();
    self.selected_piece = Some(piece);
    log(&format!("Selected piece {:?}, valid moves: {:?}", piece, valid_moves));
  }

  fn draw_piece_at(&self, x: u32, y: u32, context: &CanvasRenderingContext2d) {
    let x_pos = SQUARE_SIZE * x as f64;
    let y_pos = SQUARE_SIZE * y as f64;
    let piece = self.board.piece_at(x, y);
    match context.fill_text(&piece_to_str(piece), x_pos + 7.0, y_pos + PIECE_SIZE) {
      Err(_) => println!("Failed to write text"),
      Ok(_) => (),
    };
  }
}
