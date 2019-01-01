enum Piece {
  Rook,
  Knight,
  Bishop,
  King,
  Queen,
  Tower,
}

struct Player {
  pieces: Vec<Piece>,
}

impl Player {
  pub fn new() -> Self {
    Self {
      pieces: vec![
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Rook,
        Piece::Knight,
        Piece::Knight,
        Piece::Queen,
        Piece::King,
        Piece::Bishop,
        Piece::Bishop,
      ]
    }
  }
}

struct Game {
  player1: Player,
  player2: Player,
}

pub fn main() {
  let game = Game {
    player1: Player::new(),
    player2: Player::new(),
  };
  print_board(game);
}

fn print_board(game: Game) {
  print!("┏");
  for _ in 0..7 {
    print!("━━━┳");
  }
  println!("━━━┓");
  for y in 0..8 {
    print!("┃");
    for x in 0..8 {
      print!(" {} ┃", piece_for_cel(x, y));
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

fn piece_for_cel(x: u32, y: u32) -> String {
  if y < 2 {
    black_piece_for_col(x, y)
  } else if y > 5 {
    white_piece_for_col(x, y)
  } else {
    String::from(" ")
  }
}

fn black_piece_for_col(x: u32, y: u32) -> String {
  if y == 1 {
    String::from("♟")
  } else {
    String::from(match x {
      0 => "♜",
      1 => "♝",
      2 => "♞",
      3 => "♚",
      4 => "♛",
      5 => "♞",
      6 => "♝",
      7 => "♜",
      _ => panic!(),
    })
  }
}

fn white_piece_for_col(x: u32, y: u32) -> String {
  if y == 6 {
    String::from("♙")
  } else {
    String::from(match x {
      0 => "♖",
      1 => "♗",
      2 => "♘",
      3 => "♔",
      4 => "♕",
      5 => "♘",
      6 => "♗",
      7 => "♖",
      _ => panic!(),
    })
  }
}