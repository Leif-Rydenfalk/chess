use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub owner: usize,
}

impl Default for Piece {
    fn default() -> Self {
        Piece { kind: PieceKind::Pawn, owner: 0 }
    }
}

impl Piece {
    pub fn new(kind: PieceKind, owner: usize) -> Self {
        Piece { kind, owner }
    }
}

pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Move { from, to }
    }

    pub fn parse(input: &str) -> Option<Self> {
        let mut parts = input.split_whitespace();
        let from = parts.next()?;
        let to = parts.next()?;

        let from = (from.chars().nth(0)? as usize - 'a' as usize, from.chars().nth(1)? as usize - '1' as usize);
        let to = (to.chars().nth(0)? as usize - 'a' as usize, to.chars().nth(1)? as usize - '1' as usize);

        Some(Move::new(from, to))
    }
}

pub struct ChessBoard {
    pub grid: Grid<Option<Piece>>,
}

impl ChessBoard {
    pub fn normal_setup() -> Self {
        let mut board = ChessBoard {
            grid: Grid::new(8, 8),
        };

        // Pawns
        for col in 0..8 {
            board.set_piece(1, col, Piece::new(PieceKind::Pawn, 0));
            board.set_piece(6, col, Piece::new(PieceKind::Pawn, 1));
        }

        // Rooks
        board.set_piece(0, 0, Piece::new(PieceKind::Rook, 0));
        board.set_piece(0, 7, Piece::new(PieceKind::Rook, 0));
        board.set_piece(7, 0, Piece::new(PieceKind::Rook, 1));
        board.set_piece(7, 7, Piece::new(PieceKind::Rook, 1));

        // Knights
        board.set_piece(0, 1, Piece::new(PieceKind::Knight, 0));
        board.set_piece(0, 6, Piece::new(PieceKind::Knight, 0));
        board.set_piece(7, 1, Piece::new(PieceKind::Knight, 1));
        board.set_piece(7, 6, Piece::new(PieceKind::Knight, 1));

        // Bishops
        board.set_piece(0, 2, Piece::new(PieceKind::Bishop, 0));
        board.set_piece(0, 5, Piece::new(PieceKind::Bishop, 0));
        board.set_piece(7, 2, Piece::new(PieceKind::Bishop, 1));
        board.set_piece(7, 5, Piece::new(PieceKind::Bishop, 1));

        // Queens
        board.set_piece(0, 3, Piece::new(PieceKind::Queen, 0));
        board.set_piece(7, 3, Piece::new(PieceKind::Queen, 1));

        // Kings
        board.set_piece(0, 4, Piece::new(PieceKind::King, 0));
        board.set_piece(7, 4, Piece::new(PieceKind::King, 1));

        board
    }

    pub fn set_piece(&mut self, row: usize, col: usize, piece: Piece) {
        self.grid[(row, col)] = Some(piece);
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Option<Piece> {
        self.grid[(row, col)].clone()
    }

    pub fn remove_piece(&mut self, row: usize, col: usize) {
        self.grid[(row, col)] = None;
    }

    /// Move a piece from one position to another
    /// Returns true if the move was successful, false otherwise
    pub fn move_piece(&mut self, move_to_perform: Move) -> bool {
        let from = move_to_perform.from;
        let to = move_to_perform.to;

        // Check if from and to are within the board
        if from.0 >= self.grid.rows() || from.1 >= self.grid.cols() || to.0 >= self.grid.rows() || to.1 >= self.grid.cols() {
            println!("Invalid move");
            return false;
        }

        let piece = self.get_piece(from.0, from.1);
        if piece == None {
            println!("Could not find piece at position {:?}", from);
            return false;
        }
        let piece = piece.unwrap();

        let mut can_move = {
            let mut can_move = true;  

            // 1. Check if the destination is not the same as the source
            if from == to {
                can_move = false;
            }

            // 2. Check if the destination is within the board
            if to.0 >= self.grid.rows() || to.1 >= self.grid.cols() {
                can_move = false;
            }

            // 3. Piece-specific movement rules
            match piece.kind {
                PieceKind::Pawn => {
                    let row_diff = (to.0 as i32 - from.0 as i32).abs();
                    let col_diff = (to.1 as i32 - from.1 as i32).abs();

                    // Pawns can only move forward
                    if piece.owner == 0 && to.0 <= from.0 {
                        can_move = false;
                    }

                    if piece.owner == 1 && to.0 >= from.0 {
                        can_move = false;
                    }

                    // Pawns can only move forward by 1 or 2 squares
                    if row_diff > 2 || col_diff > 1 {
                        can_move = false;
                    }

                    // Pawns can only move diagonally if there's an enemy piece
                    if col_diff == 1 && row_diff == 1 {
                        let target_piece = self.get_piece(to.0, to.1);
                        if target_piece.is_none() || target_piece.unwrap().owner == piece.owner {
                            can_move = false;
                        }
                    }
                }
                PieceKind::Rook => {
                    let row_diff = (to.0 as i32 - from.0 as i32).abs();
                    let col_diff = (to.1 as i32 - from.1 as i32).abs();

                    // Rooks can only move in a straight line
                    if row_diff > 0 && col_diff > 0 {
                        can_move = false;
                    }
                }
                PieceKind::Knight => {
                    let row_diff = (to.0 as i32 - from.0 as i32).abs();
                    let col_diff = (to.1 as i32 - from.1 as i32).abs();

                    // Knights can move in an L-shape
                    if (row_diff == 2 && col_diff != 1) || (col_diff == 2 && row_diff != 1) {
                        can_move = false;
                    }
                }
                PieceKind::Bishop => {
                    let row_diff = (to.0 as i32 - from.0 as i32).abs();
                    let col_diff = (to.1 as i32 - from.1 as i32).abs();

                    // Bishops can only move diagonally
                    if row_diff != col_diff {
                        can_move = false;
                    }

                    // Check if the path is clear
                    let row_dir = (to.0 as i32 - from.0 as i32).signum();
                    let col_dir = (to.1 as i32 - from.1 as i32).signum();

                    let mut row = from.0 as i32 + row_dir;
                    let mut col = from.1 as i32 + col_dir;

                    while row != to.0 as i32 && col != to.1 as i32 {
                        if !self.is_empty(row as usize, col as usize) {
                            can_move = false;
                            break;
                        }

                        row += row_dir;
                        col += col_dir;
                    }
                }
                PieceKind::Queen => {
                    let row_diff = (to.0 as i32 - from.0 as i32).abs();
                    let col_diff = (to.1 as i32 - from.1 as i32).abs();

                    // Queens can move in a straight line or diagonally
                    if row_diff != col_diff && row_diff > 0 && col_diff > 0 {
                        can_move = false;
                    }

                    // Check if the path is clear
                    // Diagonal path
                    if row_diff == col_diff {
                        let row_dir = (to.0 as i32 - from.0 as i32).signum();
                        let col_dir = (to.1 as i32 - from.1 as i32).signum();

                        let mut row = from.0 as i32 + row_dir;
                        let mut col = from.1 as i32 + col_dir;

                        while row != to.0 as i32 && col != to.1 as i32 {
                            if !self.is_empty(row as usize, col as usize) {
                                can_move = false;
                                break;
                            }

                            row += row_dir;
                            col += col_dir;
                        }
                    }
                    // straight path
                    else {
                        let row_dir = (to.0 as i32 - from.0 as i32).signum();
                        let col_dir = (to.1 as i32 - from.1 as i32).signum();

                        let mut row = from.0 as i32 + row_dir;
                        let mut col = from.1 as i32 + col_dir;

                        while row != to.0 as i32 || col != to.1 as i32 {
                            if !self.is_empty(row as usize, col as usize) {
                                can_move = false;
                                break;
                            }

                            row += row_dir;
                            col += col_dir;
                        }
                    }
                }
                PieceKind::King => {
                    let row_diff = (to.0 as i32 - from.0 as i32).abs();
                    let col_diff = (to.1 as i32 - from.1 as i32).abs();

                    // Kings can only move by 1 square
                    if row_diff > 1 || col_diff > 1 {
                        can_move = false;
                    }
                }
            }

            can_move
        };

        if can_move {
            self.set_piece(to.0, to.1, piece);
            self.remove_piece(from.0, from.1);
        } else {
            println!("Invalid move");
        }   

        can_move
    }

    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.get_piece(row, col).is_none()
    }
}