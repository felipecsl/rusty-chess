use engine::piece::Color;
use engine::piece::Piece;
use engine::piece::PieceType;

#[derive(Clone)]
pub struct Player {
  pub color: Color,
  pub pieces: Vec<Piece>,
}

impl Player {
  pub fn new(color: Color) -> Player {
    let pieces = Player::init_pieces(color);
    Player { color, pieces }
  }

  pub fn update_piece_pos(&mut self, piece: &Piece, new_pos: (u32, u32)) {
    // TODO
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
}
