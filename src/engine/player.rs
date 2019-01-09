use engine::piece::Color;
use engine::piece::Piece;
use engine::piece::PieceType;

#[derive(Clone)]
pub struct Player<'a> {
  pub color: &'a Color,
  pub pieces: Vec<Piece<'a>>,
}

impl<'a> Player<'a> {
  pub fn new(color: &Color) -> Player {
    let pieces = Player::init_pieces(color);
    Player { color, pieces }
  }

  fn init_pieces(color: &'a Color) -> Vec<Piece> {
    let y_offset = if *color == Color::Black { 0 } else { 6 };
    let pawn_offset = if *color == Color::Black { 1 } else { 0 };
    let offset: i32 = if *color == Color::Black { -1 } else { 0 };
    let pawn_pos = (y_offset + pawn_offset) as u32;
    let pos = (1 + y_offset + offset) as u32;
    vec![
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (0, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (1, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (2, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (3, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (4, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (5, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (6, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Pawn,
        color,
        pos: (7, pawn_pos),
      },
      Piece {
        piece_type: PieceType::Rook,
        color,
        pos: (0, pos),
      },
      Piece {
        piece_type: PieceType::Knight,
        color,
        pos: (1, pos),
      },
      Piece {
        piece_type: PieceType::Bishop,
        color,
        pos: (2, pos),
      },
      Piece {
        piece_type: PieceType::Queen,
        color,
        pos: (3, pos),
      },
      Piece {
        piece_type: PieceType::King,
        color,
        pos: (4, pos),
      },
      Piece {
        piece_type: PieceType::Bishop,
        color,
        pos: (5, pos),
      },
      Piece {
        piece_type: PieceType::Knight,
        color,
        pos: (6, pos),
      },
      Piece {
        piece_type: PieceType::Rook,
        color,
        pos: (7, pos),
      },
    ]
  }
}
