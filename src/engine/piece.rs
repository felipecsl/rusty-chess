#[derive(PartialEq, Debug, Clone)]
pub enum Color {
  Black,
  White,
}

#[derive(Debug, Clone)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  King,
  Queen,
  Rook,
}

#[derive(Debug, Clone)]
pub struct Piece<'a> {
  pub piece_type: PieceType,
  pub color: &'a Color,
  pub pos: (u32, u32),
}

impl<'a> Piece<'a> {
  pub fn is_black(&self) -> bool {
    *self.color == Color::Black
  }
}
