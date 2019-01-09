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

  fn x(&self) -> u32 {
    self.pos.0
  }

  fn y(&self) -> u32 {
    self.pos.1
  }

  pub fn valid_moves(&self) -> Vec<(u32, u32)> {
    match self.piece_type {
      PieceType::Rook => self.rook_moves(),
      PieceType::Bishop => self.bishop_moves(),
      PieceType::Knight => self.knight_moves(),
      PieceType::King => self.king_moves(),
      PieceType::Queen => self.queen_moves(),
      PieceType::Pawn => self.pawn_moves(),
    }
  }


  fn rook_moves(&self) -> Vec<(u32, u32)> {
    vec![]
  }

  fn bishop_moves(&self) -> Vec<(u32, u32)> {
    vec![]
  }

  fn knight_moves(&self) -> Vec<(u32, u32)> {
    vec![]
  }

  fn queen_moves(&self) -> Vec<(u32, u32)> {
    vec![]
  }

  fn king_moves(&self) -> Vec<(u32, u32)> {
    vec![]
  }

  fn pawn_moves(&self) -> Vec<(u32, u32)> {
    // TODO: Handle edges of the board and first move of the game (can move 2 positions)
    return if self.is_black() {
      vec![(self.x() + 1, self.y())];
    } else {
      vec![(self.x() - 1, self.y())];
    };
  }
}
