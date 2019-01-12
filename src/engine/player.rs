use engine::piece::Color;

pub struct Player {
  pub color: Color,
}

impl Player {
  pub fn new(color: Color) -> Player {
    Player { color }
  }
}
