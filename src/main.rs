mod chess_engine;
use chess_engine::*;

mod chess_renderer;
use chess_renderer::*;

fn main() {
    let mut board = ChessBoard::normal_setup();

    board.move_piece(Move::parse("e2 e4").unwrap());

    board.move_piece(Move::new((6, 4), (4, 4)));

    board.move_piece(Move::new((1, 0), (3, 0)));

    board.move_piece(Move::new((7, 3), (3, 7)));

    ChessTextRenderer::draw_board(&board);
}
 