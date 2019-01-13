#[derive(PartialEq, Debug, Clone, Copy)]
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
pub struct Piece {
  pub piece_type: PieceType,
  pub color: Color,
  pub pos: (u32, u32),
  total_moves: u32,
}

impl Piece {
  pub fn new(piece_type: PieceType, color: Color, pos: (u32, u32)) -> Piece {
    Piece {
      piece_type,
      color,
      pos,
      total_moves: 0,
    }
  }

  pub fn to_str(&self) -> String {
    if self.is_black() {
      self.black_piece_to_str()
    } else {
      self.white_piece_to_str()
    }
  }

  fn black_piece_to_str(&self) -> String {
    String::from(match self.piece_type {
      PieceType::Rook => "♜",
      PieceType::Bishop => "♝",
      PieceType::Knight => "♞",
      PieceType::King => "♚",
      PieceType::Queen => "♛",
      PieceType::Pawn => "♟",
    })
  }

  fn white_piece_to_str(&self) -> String {
    String::from(match self.piece_type {
      PieceType::Rook => "♖",
      PieceType::Bishop => "♗",
      PieceType::Knight => "♘",
      PieceType::King => "♔",
      PieceType::Queen => "♕",
      PieceType::Pawn => "♙",
    })
  }

  pub fn is_black(&self) -> bool {
    self.color == Color::Black
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
    let mut ret = vec![];
    let x = self.x();
    let y = self.y();
    for r in 1..8 {
      ret.push((x + r, y + r));
      ret.push((x - r, y + r));
      ret.push((x + r, y - r));
      ret.push((x - r, y - r));
    }
    return ret;
  }

  fn knight_moves(&self) -> Vec<(u32, u32)> {
    // TODO: Handle edges of the board and piece collision
    let mut ret = vec![];
    let x = self.x();
    let y = self.y();
    if self.is_black() {
      ret.extend(&[
        (x - 1, y + 2),
        (x + 1, y + 2),
        (x - 1, y - 2),
        (x + 1, y - 2),
        (x - 2, y + 1),
        (x + 2, y + 1),
        (x - 2, y - 1),
        (x + 2, y - 1),
      ]);
    } else {
      ret.extend(&[
        (x - 1, y + 2),
        (x + 1, y + 2),
        (x - 1, y - 2),
        (x + 1, y - 2),
        (x - 2, y + 1),
        (x + 2, y + 1),
        (x - 2, y - 1),
        (x + 2, y - 1),
      ]);
    }
    return ret;
  }

  fn queen_moves(&self) -> Vec<(u32, u32)> {
    vec![]
  }

  fn king_moves(&self) -> Vec<(u32, u32)> {
    vec![]
  }

  fn pawn_moves(&self) -> Vec<(u32, u32)> {
    // TODO: Handle edges of the board and piece collision
    let x = self.x();
    let y = self.y();
    let extra_move = if self.total_moves == 0 { true } else { false };
    let mut ret = vec![];
    if self.is_black() {
      ret.push((x, y + 1));
      if extra_move {
        ret.push((x, y + 2));
      }
    } else {
      ret.push((x, y - 1));
      if extra_move {
        ret.push((x, y - 2));
      }
    };
    return ret;
  }
}
