mod engine;

use engine::player::Player;
use engine::board::Board;
use engine::piece::Color;

pub fn main() {
  let board = Board {
    player1: Player::new(&Color::Black),
    player2: Player::new(&Color::White),
  };
  board.print();
}