#[derive(PartialEq, Debug, Clone)]
pub enum Color {
  Black,
  White,
}

#[derive(PartialEq, Debug, Clone)]
pub enum PieceType {
  Pawn,
  Knight,
  Bishop,
  King,
  Queen,
  Rook,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Piece<'a> {
  pub piece_type: PieceType,
  pub color: &'a Color,
  pub pos: (u32, u32),
  total_moves: u32,
}

impl<'a> Piece<'a> {
  pub fn new(piece_type: PieceType, color: &Color, pos: (u32, u32)) -> Piece {
    Piece {
      piece_type,
      color,
      pos,
      total_moves: 0
    }
  }

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
    // TODO: Handle edges of the board
    let x = self.x();
    let extra_move = if self.total_moves == 0 { true } else { false };
    let mut ret = vec![];
    if self.is_black() {
      ret.push((x, self.y() + 1));
      if extra_move {
        ret.push((x, self.y() + 2));
      }
    } else {
      ret.push((x, self.y() - 1));
      if extra_move {
        ret.push((x, self.y() - 2));
      }
    };
    return ret;
  }
}
