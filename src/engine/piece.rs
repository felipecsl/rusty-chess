#[derive(PartialEq)]
pub enum Color {
  Black, White
}

pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  King,
  Queen,
  Rook,
}

pub struct Piece {
  pub piece_type: PieceType,
  pub color: &'static Color,
  pub pos: (u32, u32),
}

impl Piece {
  pub fn is_black(&self) -> bool {
    *self.color == Color::Black
  }
}
