use engine::piece::Piece;

pub fn piece_to_str(piece: Option<&Piece>) -> String {
  match piece {
    Some(ref piece) => piece.to_str(),
    None => String::from(" "),
  }
}
