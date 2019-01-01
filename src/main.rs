mod piece;
mod player;
mod board;

use piece::Color;
use player::Player;
use board::Board;

pub fn main() {
  let board = Board {
    player1: Player::new(&Color::Black),
    player2: Player::new(&Color::White),
  };
  board.print();
}