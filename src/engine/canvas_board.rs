extern crate wasm_bindgen;
extern crate web_sys;

use engine::board::*;
use engine::piece::Piece;
use log;
use PIECE_SIZE;
use SQUARE_SIZE;

use self::wasm_bindgen::prelude::*;
use self::web_sys::CanvasRenderingContext2d;

static COLOR_1: &str = "#f4d9b0";
static COLOR_2: &str = "#bc865c";
static COLOR_SELECTED_1: &str = "#c1a680";
static COLOR_SELECTED_2: &str = "#895329";

#[allow(dead_code)]
pub struct CanvasBoardRenderer<'a> {
  board: &'a Board<'a>,
  selected_piece: Option<&'a Piece<'a>>,
  valid_moves: Vec<(u32, u32)>,
}

impl<'a> CanvasBoardRenderer<'a> {
  pub fn new(board: &'a Board<'a>) -> CanvasBoardRenderer<'a> {
    CanvasBoardRenderer {
      board,
      selected_piece: None,
      valid_moves: vec![],
    }
  }

  #[allow(dead_code)]
  pub fn render(&self, context: &CanvasRenderingContext2d) {
    context.set_font(&format!("{}px Courier New", PIECE_SIZE));
    for y in 0..8 {
      for x in 0..8 {
        let option_1 = (x + y) % 2 == 0;
        let color = if option_1 { COLOR_1 } else { COLOR_2 };
        let color_selected = if option_1 { COLOR_SELECTED_1 } else { COLOR_SELECTED_2 };
        let x_pos = SQUARE_SIZE * x as f64;
        let y_pos = SQUARE_SIZE * y as f64;
        let piece = self.board.piece_at(x, y);
        if (self.valid_moves.contains(&(x, y)) && piece == None) || (piece != None && self.selected_piece == piece) {
          context.set_fill_style(&JsValue::from(color_selected));
        } else {
          context.set_fill_style(&JsValue::from(color));
        }
        context.fill_rect(x_pos, y_pos, SQUARE_SIZE, SQUARE_SIZE);
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

  /* Marks the provided piece as currently selected */
  pub fn select_piece(&mut self, piece: &'a Piece<'a>) {
    self.valid_moves.clear();
    if Some(piece) == self.selected_piece {
      // Clear selected piece if reselected
      self.selected_piece = None
    } else {
      let valid_moves = piece.valid_moves();
      log(&format!("Selected piece {:?}, valid moves: {:?}", piece, valid_moves));
      self.selected_piece = Some(piece);
      self.valid_moves.extend(valid_moves);
    }
  }

  fn draw_piece_at(&self, x: u32, y: u32, context: &CanvasRenderingContext2d) {
    let x_pos = SQUARE_SIZE * x as f64;
    let y_pos = SQUARE_SIZE * y as f64;
    let piece = self.board.piece_at(x, y);
    match context.fill_text(&piece_to_str(piece), x_pos + 7.0, y_pos + PIECE_SIZE) {
      Err(_) => log("Failed to write text"),
      Ok(_) => (),
    };
  }
}
