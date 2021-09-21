/*
 * Schackmotor
 * @author Gustaf Haglund <ghaglu@kth.se>
 */

use array2d::Array2D;
use std::fs;

const _WHITE_PIECE:usize = 0;
const _BLACK_PIECE:usize = 1;

#[derive(Debug, Clone)]
struct Move {
	before: Vec<u32>,
	after: Vec<u32> 
}

/*struct Position {
	row: u32,
	column: u32
}*/

#[derive(Debug, Clone)]
struct Piece {
	//position: Position,
	identity: String,
	color: usize
}

#[derive(Debug, Clone)]
pub struct Board {
	table: Array2D<Piece>,
	history: Vec<Move>,
}

pub fn initialize_board() -> Board
{
	let null_piece = Piece {identity: "".to_string(), color: _WHITE_PIECE};
	let mut board = Board { table: Array2D::filled_with(null_piece, 8, 8), history: vec![] };
	let mut row:usize = 0;
	let mut column:usize = 0;
	let mut iter = 0;

	let contents = fs::read_to_string("/Users/gustaf/Desktop/lekis/plusplus/schackmotor/board_definition").expect("Something went wrong reading the file");
	for c in contents.replace("\n", "").chars() {
		if iter == 8 {
			row += 1;
			column = 0;
			iter = 0;
		}
		board.table[(row, column)].identity = c.to_string();
		board.table[(row, column)].color = _WHITE_PIECE;
		board.table[(7 - row, column)].identity = c.to_string();
		board.table[(7 - row, column)].color = _BLACK_PIECE;
		column += 1;
		iter += 1;
	}

	board
}

trait Piece {
	fn theory_valid_move(Vec) -> bool;
}

trait Notation {
	fn use(Board, String) -> Board;
}

//struct AlgebraicNotation {}

impl Piece for Pawn {
	fn theory_valid_move(move: Vec) -> bool {

	}
}

impl Notation for LongAlgebraicNotation {
	fn use(board: Board, move: String) -> Board {

		String startPiece = move.chars()[..move.len()-(move.len()-2)];
		String endPiece = move.chars()[move.len() - (move.len()-2)..];
		
		board
	}
}


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

	#[test]
	fn board_loaded() {
		let board = initialize_board();
		println!("{:?}", board);
	}
}