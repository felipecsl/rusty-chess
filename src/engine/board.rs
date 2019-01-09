use engine::piece::Color;
use engine::piece::Piece;
use engine::piece::PieceType;
use engine::player::Player;

#[derive(Clone)]
pub struct Board<'a> {
  player1: Player<'a>,
  player2: Player<'a>,
}

impl<'a> Board<'a> {
  pub fn new() -> Board<'a> {
    let player1 = Player::new(&Color::Black);
    let player2 = Player::new(&Color::White);
    Board { player1, player2 }
  }

  pub fn piece_at(&self, x: u32, y: u32) -> Option<&Piece> {
    let all_pieces = self.all_pieces();
    let matches = all_pieces
      .iter()
      .filter(|&p| p.pos == (x, y))
      .collect::<Vec<&&Piece>>();
    return matches.first().map(|&p| *p);
  }

  fn all_pieces(&self) -> Vec<&Piece> {
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
