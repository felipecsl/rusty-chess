extern crate wasm_bindgen;
extern crate web_sys;

use engine::canvas_board::CanvasBoardRenderer;
use engine::canvas_board::piece_at;
use engine::piece::Color;
use engine::piece::Piece;
use engine::piece::PieceType;
use engine::player::Player;

use self::web_sys::HtmlTextAreaElement;

pub struct GameController<'a> {
  renderer: &'a mut CanvasBoardRenderer,
  status: HtmlTextAreaElement,
  current_player_color: Color,
  player1: Player,
  player2: Player,
  all_pieces: Vec<Piece>,
}

impl<'a> GameController<'a> {
  pub fn new(
    renderer: &'a mut CanvasBoardRenderer,
    status: HtmlTextAreaElement,
  ) -> GameController<'a> {
    let player1 = Player::new(Color::Black);
    let player2 = Player::new(Color::White);
    let all_pieces = all_pieces();
    GameController {
      renderer,
      status,
      current_player_color: Color::White,
      player1,
      player2,
      all_pieces,
    }
  }

  pub fn render(&self) {
    self.renderer.render(&self.all_pieces);
  }

  pub fn handle_click_at(&mut self, x: u32, y: u32) {
    let all_pieces = self.all_pieces.clone();
    let target_piece = piece_at(&all_pieces, x, y);
    if target_piece == None {
      // destination is "empty", just try to move if that's a valid move for the selected piece
      self.maybe_move(&all_pieces, x, y);
    } else if self.is_capture(target_piece) {
      // we're capturing a piece
      self.maybe_move_and_capture(&all_pieces, target_piece.unwrap(), x, y);
    } else if self.current_player_color == target_piece.unwrap().color {
      // select a new piece
      // make sure the piece we're selecting matches the player color who's currently playing
      self.renderer.select_piece(target_piece.cloned());
    }
    // Re-draw the board to reflect moved/selected piece
    self.render();
  }

  /** Are we trying to capture an enemy piece? */
  fn is_capture(&self, target_piece: Option<&Piece>) -> bool {
    let selected_piece = self.renderer.selected_piece();
    selected_piece != None
      && target_piece != selected_piece
      && target_piece.unwrap().color != selected_piece.unwrap().color
  }

  fn maybe_move_and_capture(&mut self, all_pieces: &Vec<Piece>, target_piece: &Piece, x: u32, y: u32) {
    if self.renderer.can_selected_piece_move_or_capture_to(all_pieces, x, y) {
      self.move_selected_piece_to(x, y);
      self.remove_piece_from_board(target_piece);
      self.append_log(&format!("Piece captured {:?}", target_piece));
      self.append_log(&format!("Next turn {:?}", self.current_player_color));
    }
  }

  fn remove_piece_from_board(&mut self, piece: &Piece) {
    if let Some(pos) = self.all_pieces.iter().position(|x| *x == *piece) {
      self.all_pieces.remove(pos);
    }
  }

  fn maybe_move(&mut self, all_pieces: &Vec<Piece>, x: u32, y: u32) {
    if self.renderer.can_selected_piece_move_or_capture_to(all_pieces, x, y) {
      {
        let piece = self.renderer.selected_piece().unwrap();
        self.append_log(&format!("Moving {:?} to {:?}", piece, (x, y)));
      }
      self.move_selected_piece_to(x, y);
      self.append_log(&format!("Next turn {:?}", self.current_player_color));
    }
  }

  fn append_log(&self, text: &str) {
    self
      .status
      .set_value(&format!("{}{}{}", text, "\n", self.status.value()));
  }

  fn move_selected_piece_to(&mut self, x: u32, y: u32) {
    {
      let selected_piece = self.renderer.selected_piece().unwrap();
      let p = self
        .all_pieces
        .iter_mut()
        .find(|p| p.pos == selected_piece.pos)
        .unwrap();
      p.move_to(x, y);
    }
    self.renderer.unselect_piece();
    self.current_player_color = if self.current_player_color == Color::White {
      Color::Black
    } else {
      Color::White
    };
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
