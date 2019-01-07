use engine::piece::Piece;
use engine::piece::PieceType;
use engine::player::Player;
use engine::piece::Color;

#[derive(Clone)]
pub struct Board {
  pub player1: Player,
  pub player2: Player,
}

impl Board {
  fn all_pieces(&self) -> Vec<&Piece> {
    let mut all_pieces = Vec::with_capacity(32);
    all_pieces.extend(&self.player1.pieces);
    all_pieces.extend(&self.player2.pieces);
    return all_pieces;
  }

  pub fn piece_at<'a>(
      &'a self,
      x: u32,
      y: u32
  ) -> Option<&Piece> {
    let all_pieces = self.all_pieces();
    let matches = all_pieces.iter()
      .filter(|&p| p.pos == (x, y))
      .collect::<Vec<&&Piece>>();
    return matches.first().map(|&p| *p);
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
    },
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

pub fn new_board() -> Board {
  Board {
    player1: Player::new(&Color::Black),
    player2: Player::new(&Color::White),
  }
}