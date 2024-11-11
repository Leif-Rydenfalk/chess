use crate::chess_engine::{ChessBoard, Piece, PieceKind};

pub struct ChessTextRenderer {}

impl ChessTextRenderer {
    pub fn draw_board(board: &ChessBoard) {
       for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board.get_piece(row, col) {
                    match piece.kind {
                        PieceKind::Pawn => print!("P"),
                        PieceKind::Rook => print!("R"),
                        PieceKind::Knight => print!("N"),
                        PieceKind::Bishop => print!("B"),
                        PieceKind::Queen => print!("Q"),
                        PieceKind::King => print!("K"),
                    }
                } else {
                    print!(".");
                }
                print!(" ");
            }
            println!();
        }
    }
}
