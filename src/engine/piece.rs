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

  pub fn move_to(&mut self, x: u32, y: u32) {
    self.pos = (x, y);
    self.total_moves += 1;
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

  pub fn valid_moves(&self, blocked_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    match self.piece_type {
      PieceType::Rook => self.rook_moves(blocked_positions),
      PieceType::Bishop => self.bishop_moves(blocked_positions),
      PieceType::Knight => self.knight_moves(),
      PieceType::King => self.king_moves(),
      PieceType::Queen => self.queen_moves(blocked_positions),
      PieceType::Pawn => self.pawn_moves(),
    }
  }

  pub fn valid_captures(&self, opponent_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    match self.piece_type {
      PieceType::Rook => self.rook_captures(opponent_positions),
      PieceType::Bishop => self.bishop_captures(opponent_positions),
      PieceType::Knight => self.knight_captures(opponent_positions),
      PieceType::King => self.king_captures(opponent_positions),
      PieceType::Queen => self.queen_captures(opponent_positions),
      PieceType::Pawn => self.pawn_captures(opponent_positions),
    }
  }

  fn rook_captures(&self, opponent_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    vec![]
  }

  fn bishop_captures(&self, opponent_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    vec![]
  }

  fn knight_captures(&self, opponent_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    self.knight_moves()
      .into_iter()
      .filter(|&m| opponent_positions.contains(&m))
      .collect()
  }

  fn king_captures(&self, opponent_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    vec![]
  }

  fn queen_captures(&self, opponent_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    vec![]
  }

  fn pawn_captures(&self, opponent_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut ret = vec![];
    let x = self.x();
    let y = self.y();
    if self.is_black() {
      if opponent_positions.contains(&(x + 1, y + 1)) {
        ret.push((x + 1, y + 1));
      }
      if opponent_positions.contains(&(x - 1, y + 1)) {
        ret.push((x - 1, y + 1));
      }
    } else {
      if opponent_positions.contains(&(x + 1, y - 1)) {
        ret.push((x + 1, y - 1));
      }
      if opponent_positions.contains(&(x - 1, y - 1)) {
        ret.push((x - 1, y - 1));
      }
    }
    return ret;
  }

  fn rook_moves(&self, blocked_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut ret = vec![];
    let x = self.x();
    let y = self.y();
    let mut right_blocked = false;
    let mut left_blocked = false;
    let mut top_blocked = false;
    let mut bottom_blocked = false;
    for r in 1..8 {
      let pos_bottom = (x, y + r);
      if !bottom_blocked {
        bottom_blocked = blocked_positions.contains(&pos_bottom);
        if !bottom_blocked {
          ret.push(pos_bottom);
        }
      }
      let pos_left = (x - r, y);
      if !left_blocked {
        left_blocked = blocked_positions.contains(&pos_left);
        if !left_blocked {
          ret.push(pos_left);
        }
      }
      let pos_right = (x + r, y);
      if !right_blocked {
        right_blocked = blocked_positions.contains(&pos_right);
        if !right_blocked {
          ret.push(pos_right);
        }
      }
      let pos_top = (x, y - r);
      if !top_blocked {
        top_blocked = blocked_positions.contains(&pos_top);
        if !top_blocked {
          ret.push(pos_top);
        }
      }
    }
    return ret;
  }

  fn bishop_moves(&self, blocked_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut ret = vec![];
    let x = self.x();
    let y = self.y();
    let mut bottom_right_blocked = false;
    let mut bottom_left_blocked = false;
    let mut top_right_blocked = false;
    let mut top_left_blocked = false;
    for r in 1..8 {
      let pos_bottom_right = (x + r, y + r);
      if !bottom_right_blocked {
        bottom_right_blocked = blocked_positions.contains(&pos_bottom_right);
        if !bottom_right_blocked {
          ret.push(pos_bottom_right);
        }
      }
      let pos_bottom_left = (x - r, y + r);
      if !bottom_left_blocked {
        bottom_left_blocked = blocked_positions.contains(&pos_bottom_left);
        if !bottom_left_blocked {
          ret.push(pos_bottom_left);
        }
      }
      let pos_top_right = (x + r, y - r);
      if !top_right_blocked {
        top_right_blocked = blocked_positions.contains(&pos_top_right);
        if !top_right_blocked {
          ret.push(pos_top_right);
        }
      }
      let pos_top_left = (x - r, y - r);
      if !top_left_blocked {
        top_left_blocked = blocked_positions.contains(&pos_top_left);
        if !top_left_blocked {
          ret.push(pos_top_left);
        }
      }
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

  fn queen_moves(&self, blocked_positions: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut ret = self.bishop_moves(blocked_positions);
    ret.append(&mut self.rook_moves(blocked_positions));
    return ret;
  }

  fn king_moves(&self) -> Vec<(u32, u32)> {
    let x = self.x();
    let y = self.y();
    vec![
      (x - 1, y),
      (x + 1, y),
      (x, y - 1),
      (x, y + 1),
      (x + 1, y + 1),
      (x - 1, y - 1),
      (x - 1, y + 1),
      (x + 1, y - 1),
    ]
  }

  fn pawn_moves(&self) -> Vec<(u32, u32)> {
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
