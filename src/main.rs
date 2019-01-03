mod engine;

pub fn main() {
  let board = engine::board::new_board();
  board.print_to_stdout();
}