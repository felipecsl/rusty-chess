use engine::piece::Color;
use engine::piece::Piece;
use engine::piece::PieceType;
use engine::player::Player;

#[derive(Clone)]
pub struct Board {
  player1: Player,
  player2: Player,
}

impl Board {
  pub fn new() -> Board {
    let player1 = Player::new(Color::Black);
    let player2 = Player::new(Color::White);
    Board { player1, player2 }
  }

  pub fn update_piece_pos(&mut self, piece: &Piece, new_pos: (u32, u32)) {
    if self.player1.pieces.contains(piece) {
      self.player1.update_piece_pos(piece, new_pos);
    } else {
      self.player2.update_piece_pos(piece, new_pos);
    }
  }

  pub fn all_pieces(&self) -> Vec<&Piece> {
    let p1 = &self.player1.pieces;
    let p2 = &self.player2.pieces;
    let mut all_pieces = Vec::with_capacity(p1.len() + p2.len());
    all_pieces.extend(p1);
    all_pieces.extend(p2);
    return all_pieces;
  }
}

pub fn piece_to_str(piece: Option<&Piece>) -> String {
  match piece {
    Some(ref piece) => {
      if piece.is_black() {
        black_piece_to_str(&piece)
      } else {
        white_piece_to_str(&piece)
      }
    }
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
