extern crate wasm_bindgen;
extern crate web_sys;

use engine::canvas_board::piece_at;
use engine::canvas_board::CanvasBoardRenderer;
use engine::piece::Color;
use engine::piece::Piece;
use engine::piece::PieceType;
use engine::player::Player;

pub struct GameController<'a> {
  renderer: &'a mut CanvasBoardRenderer,
  player1: Player,
  player2: Player,
  all_pieces: Vec<Piece>,
}

impl<'a> GameController<'a> {
  pub fn new(renderer: &'a mut CanvasBoardRenderer) -> GameController<'a> {
    let player1 = Player::new(Color::Black);
    let player2 = Player::new(Color::White);
    let all_pieces = all_pieces();
    GameController {
      renderer,
      player1,
      player2,
      all_pieces,
    }
  }

  pub fn render(&self) {
    self.renderer.render(&self.all_pieces);
  }

  pub fn handle_click_at(&mut self, x: u32, y: u32) {
    let all_pieces_clone = self.all_pieces.clone();
    let piece = piece_at(&all_pieces_clone, x, y);
    if piece == None {
      if self.renderer.can_selected_piece_move_to(&all_pieces_clone, x, y) {
        self.move_selected_piece_to(x, y);
      }
    } else {
      self.renderer.select_piece(piece.cloned());
    }
    // Re-draw the board to reflect moved/selected piece
    self.render();
  }

  fn move_selected_piece_to(&mut self, x: u32, y: u32) {
    {
      let selected_piece = self.renderer.selected_piece().unwrap();
      let p = self
        .all_pieces
        .iter_mut()
        .find(|p| p.pos == selected_piece.pos)
        .unwrap();
      p.pos = (x, y);
    }
    self.renderer.unselect_piece();
  }
}

fn all_pieces() -> Vec<Piece> {
  let mut ret = init_pieces(Color::Black);
  let mut p2 = init_pieces(Color::White);
  ret.append(&mut p2);
  return ret;
}

fn init_pieces(color: Color) -> Vec<Piece> {
  let y_offset = if color == Color::Black { 0 } else { 6 };
  let pawn_offset = if color == Color::Black { 1 } else { 0 };
  let offset: i32 = if color == Color::Black { -1 } else { 0 };
  let pawn_pos = (y_offset + pawn_offset) as u32;
  let pos = (1 + y_offset + offset) as u32;
  vec![
    Piece::new(PieceType::Pawn, color, (0, pawn_pos)),
    Piece::new(PieceType::Pawn, color, (1, pawn_pos)),
    Piece::new(PieceType::Pawn, color, (2, pawn_pos)),
    Piece::new(PieceType::Pawn, color, (3, pawn_pos)),
    Piece::new(PieceType::Pawn, color, (4, pawn_pos)),
    Piece::new(PieceType::Pawn, color, (5, pawn_pos)),
    Piece::new(PieceType::Pawn, color, (6, pawn_pos)),
    Piece::new(PieceType::Pawn, color, (7, pawn_pos)),
    Piece::new(PieceType::Rook, color, (0, pos)),
    Piece::new(PieceType::Knight, color, (1, pos)),
    Piece::new(PieceType::Bishop, color, (2, pos)),
    Piece::new(PieceType::Queen, color, (3, pos)),
    Piece::new(PieceType::King, color, (4, pos)),
    Piece::new(PieceType::Bishop, color, (5, pos)),
    Piece::new(PieceType::Knight, color, (6, pos)),
    Piece::new(PieceType::Rook, color, (7, pos)),
  ]
}
