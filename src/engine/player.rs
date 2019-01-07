use engine::piece::Piece;
use engine::piece::PieceType;
use engine::piece::Color;

#[derive(Clone)]
pub struct Player {
  pub pieces: Vec<Piece>,
}

impl Player {
  pub fn new(color: &'static Color) -> Self {
    let y_offset = if *color == Color::Black { 0 } else { 6 };
    let pawn_offset = if *color == Color::Black { 1 } else { 0 };
    let offset: i32 = if *color == Color::Black { -1 } else { 0 };
    let pawn_pos = (y_offset + pawn_offset) as u32;
    let pos = (1 + y_offset + offset) as u32;
    Self {
      pieces: vec![
        Piece { piece_type: PieceType::Pawn, color: color, pos: (0, pawn_pos) },
        Piece { piece_type: PieceType::Pawn, color: color, pos: (1, pawn_pos) },
        Piece { piece_type: PieceType::Pawn, color: color, pos: (2, pawn_pos) },
        Piece { piece_type: PieceType::Pawn, color: color, pos: (3, pawn_pos) },
        Piece { piece_type: PieceType::Pawn, color: color, pos: (4, pawn_pos) },
        Piece { piece_type: PieceType::Pawn, color: color, pos: (5, pawn_pos) },
        Piece { piece_type: PieceType::Pawn, color: color, pos: (6, pawn_pos) },
        Piece { piece_type: PieceType::Pawn, color: color, pos: (7, pawn_pos) },
        Piece { piece_type: PieceType::Rook, color: color, pos: (0, pos) },
        Piece { piece_type: PieceType::Knight, color: color, pos: (1, pos) },
        Piece { piece_type: PieceType::Bishop, color: color, pos: (2, pos) },
        Piece { piece_type: PieceType::Queen, color: color, pos: (3, pos) },
        Piece { piece_type: PieceType::King, color: color, pos: (4, pos) },
        Piece { piece_type: PieceType::Bishop, color: color, pos: (5, pos) },
        Piece { piece_type: PieceType::Knight, color: color, pos: (6, pos) },
        Piece { piece_type: PieceType::Rook, color: color, pos: (7, pos) },
      ]
    }
  }
}