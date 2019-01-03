extern crate wasm_bindgen;

use engine::piece::Piece;
use engine::piece::PieceType;
use engine::player::Player;
use engine::piece::Color;
use engine::board::wasm_bindgen::prelude::*;

pub struct Board {
  pub player1: Player,
  pub player2: Player,
}

impl Board {
  pub fn print_to_stdout(&self) {
    let all_pieces = self.all_pieces();
    print!("┏");
    for _ in 0..7 {
      print!("━━━┳");
    }
    println!("━━━┓");
    for y in 0..8 {
      print!("┃");
      for x in 0..8 {
        let piece = self.piece_at(&all_pieces, x, y);
        print!(" {} ┃", piece_to_str(piece));
      }
      println!();
      if y < 7 {
        print!("┣");
        for _ in 0..7 {
          print!("━━━╋");
        }
        print!("━━━┫");
        println!();
      }
    }
    print!("┗");
    for _ in 0..7 {
      print!("━━━┻");
    }
    println!("━━━┛");
    println!();
  }

  pub fn print_to_canvas(&self, context: &web_sys::CanvasRenderingContext2d) {
    let all_pieces = self.all_pieces();
    let canvas_size = 600.0;
    let piece_size = canvas_size / 10.0;
    let color_1 = "#f4d9b0";
    let color_2 = "#bc865c";
    let square_size = canvas_size / 8.0;
    context.set_font(&format!("{}px Courier New", piece_size));
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
        let x_pos = square_size * x as f64;
        let y_pos = square_size * y as f64;
        let piece = self.piece_at(&all_pieces, x, y);
        match context.fill_text(&piece_to_str(piece), x_pos + 7.0, y_pos + piece_size) {
          Err(_) => println!("Failed to write text"),
          Ok(_) => (),
        };
      }
    }
    for y in 6..8 {
      for x in 0..8 {
        let x_pos = square_size * x as f64;
        let y_pos = square_size * y as f64;
        let piece = self.piece_at(&all_pieces, x, y);
        match context.fill_text(&piece_to_str(piece), x_pos + 7.0, y_pos + piece_size) {
          Err(_) => println!("Failed to write text"),
          Ok(_) => (),
        };
      }
    }
  }

  fn all_pieces(&self) -> Vec<&Piece> {
    let mut all_pieces = Vec::with_capacity(32);
    all_pieces.extend(&self.player1.pieces);
    all_pieces.extend(&self.player2.pieces);
    return all_pieces;
  }

  fn piece_at<'a>(&'a self, all_pieces: &'a Vec<&Piece>, x: u32, y: u32) -> Option<&Piece> {
    let matches = all_pieces.iter()
      .filter(|&p| p.pos == (x, y))
      .collect::<Vec<&&Piece>>();
    return matches.first().map(|&p| *p);
  }
}

fn piece_to_str(piece: Option<&Piece>) -> String {
  match piece {
    Some(ref piece) => {
      if piece.is_black() {
        black_piece_to_str(&piece)
      } else {
        white_piece_to_str(&piece)
      }
    },
    None => String::from(" "),
  }
}

fn black_piece_to_str(piece: &Piece) -> String {
  String::from(match piece.piece_type {
    PieceType::Rook => "♜",
    PieceType::Bishop => "♝",
    PieceType::Knight => "♞",
    PieceType::King => "♚",
    PieceType::Queen => "♛",
    PieceType::Pawn => "♟",
  })
}

fn white_piece_to_str(piece: &Piece) -> String {
  String::from(match piece.piece_type {
    PieceType::Rook => "♖",
    PieceType::Bishop => "♗",
    PieceType::Knight => "♘",
    PieceType::King => "♔",
    PieceType::Queen => "♕",
    PieceType::Pawn => "♙",
  })
}

pub fn new_board() -> Board {
  Board {
    player1: Player::new(&Color::Black),
    player2: Player::new(&Color::White),
  }
}