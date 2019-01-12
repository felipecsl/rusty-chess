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
static COLOR_SELECTED_1: &str = "#f6fbd3";
static COLOR_SELECTED_2: &str = "#895329";

#[allow(dead_code)]
pub struct CanvasBoardRenderer {
  context: CanvasRenderingContext2d,
  selected_piece: Option<Piece>,
}

impl CanvasBoardRenderer {
  pub fn new(context: CanvasRenderingContext2d) -> CanvasBoardRenderer {
    CanvasBoardRenderer {
      context,
      selected_piece: None,
    }
  }

  #[allow(dead_code)]
  pub fn render(&self, all_pieces: &Vec<Piece>) {
    self
      .context
      .set_font(&format!("{}px Courier New", PIECE_SIZE));
    let selected_piece = self.selected_piece();
    let valid_moves = match selected_piece {
      Some(p) => p.valid_moves(),
      None => vec![],
    };
    for y in 0..8 {
      for x in 0..8 {
        let option_1 = (x + y) % 2 == 0;
        let color = if option_1 { COLOR_1 } else { COLOR_2 };
        let color_selected = if option_1 {
          COLOR_SELECTED_1
        } else {
          COLOR_SELECTED_2
        };
        let x_pos = SQUARE_SIZE * x as f64;
        let y_pos = SQUARE_SIZE * y as f64;
        let piece = piece_at(all_pieces, x, y);
        // -> position highlighting rules:
        // 1. only highlight position if it's currently vacant (no piece already there)
        // 2. mark the position of the currently selected piece (if any)
        if (valid_moves.contains(&(x, y)) && piece == None)
          || (piece != None
            && selected_piece != None
            && selected_piece.unwrap().pos == piece.unwrap().pos)
        {
          self.context.set_fill_style(&JsValue::from(color_selected));
        } else {
          self.context.set_fill_style(&JsValue::from(color));
        }
        self
          .context
          .fill_rect(x_pos, y_pos, SQUARE_SIZE, SQUARE_SIZE);
      }
    }
    for piece in all_pieces {
      let color = if piece.is_black() {
        JsValue::from("black")
      } else {
        JsValue::from("white")
      };
      self.context.set_fill_style(&color);
      let x = piece.pos.0;
      let y = piece.pos.1;
      let x_pos = SQUARE_SIZE * x as f64;
      let y_pos = SQUARE_SIZE * y as f64;
      let text = &piece_to_str(Some(piece));
      match self
        .context
        .fill_text(text, x_pos + 7.0, y_pos + PIECE_SIZE)
      {
        Err(_) => log("Failed to write text"),
        Ok(_) => (),
      };
    }
  }

  pub fn selected_piece(&self) -> Option<&Piece> {
    self.selected_piece.as_ref()
  }

  /* Marks the provided piece as currently selected */
  pub fn select_piece(&mut self, piece: Option<Piece>) {
    if piece == self.selected_piece {
      // Clear selected piece if reselected
      self.selected_piece = None
    } else {
      self.selected_piece = piece;
      //log(&format!("Selected piece {:?}, valid moves: {:?}", self.selected_piece, valid_moves));
    }
  }

  pub fn can_selected_piece_move_to(&self, x: u32, y: u32) -> bool {
    if let Some(ref piece) = self.selected_piece {
      let valid_moves = self.selected_piece().unwrap().valid_moves();
      if valid_moves.contains(&(x, y)) {
        return true;
      }
    }
    return false;
  }
}

pub fn piece_at(all_pieces: &Vec<Piece>, x: u32, y: u32) -> Option<&Piece> {
  return all_pieces.iter().find(|&p| p.pos == (x, y));
}
