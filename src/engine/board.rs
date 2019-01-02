use engine::piece::Piece;
use engine::piece::PieceType;
use engine::player::Player;

pub struct Board {
  pub player1: Player,
  pub player2: Player,
}

impl Board {
  pub fn print(&self) {
    let all_pieces = self.all_pieces();
    print!("┏");
    for _ in 0..7 {
      print!("━━━┳");
    }
    println!("━━━┓");
    for y in 0..8 {
      print!("┃");
      for x in 0..8 {
        let piece = self.piece_at(&all_pieces, x, y);
        print!(" {} ┃", piece_to_str(piece));
      }
      println!();
      if y < 7 {
        print!("┣");
        for _ in 0..7 {
          print!("━━━╋");
        }
        print!("━━━┫");
        println!();
      }
    }
    print!("┗");
    for _ in 0..7 {
      print!("━━━┻");
    }
    println!("━━━┛");
    println!();
  }

  fn all_pieces(&self) -> Vec<&Piece> {
    let mut all_pieces = Vec::with_capacity(32);
    all_pieces.extend(&self.player1.pieces);
    all_pieces.extend(&self.player2.pieces);
    return all_pieces;
  }

  fn piece_at<'a>(&'a self, all_pieces: &'a Vec<&Piece>, x: u32, y: u32) -> Option<&Piece> {
    let matches = all_pieces.iter()
      .filter(|&p| p.pos == (x, y))
      .collect::<Vec<&&Piece>>();
    return matches.first().map(|&p| *p);
  }
}

fn piece_to_str(piece: Option<&Piece>) -> String {
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