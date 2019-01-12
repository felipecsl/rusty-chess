extern crate wasm_bindgen;
extern crate web_sys;

use engine::board::Board;
use engine::canvas_board::piece_at;
use engine::canvas_board::CanvasBoardRenderer;

pub struct GameController<'a> {
  renderer: &'a mut CanvasBoardRenderer<'a>,
  board: &'a Board,
}

impl<'a> GameController<'a> {
  pub fn new(
    renderer: &'a mut CanvasBoardRenderer<'a>,
    board: &'a mut Board,
  ) -> GameController<'a> {
    GameController { renderer, board }
  }

  pub fn render(&self) {
    self.renderer.render(&self.board.all_pieces());
  }

  pub fn handle_click_at(&mut self, x: u32, y: u32) {
    let all_pieces = self.board.all_pieces();
    let piece = piece_at(&all_pieces, x, y);
    if piece == None {
      if self.renderer.can_selected_piece_move_to(x, y) {
        //        self.board.update_piece_pos(self.renderer.selected_piece.unwrap(), (x, y));
      }
    } else {
      self.renderer.select_piece(piece);
    }
    // Re-draw the board to reflect moved/selected piece
    self.render();
  }
}
