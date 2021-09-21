use schackmotor::*;

fn main() {
	let board = initialize_board();
	println!("{:?}", board);

	String startPiece = move.chars()[..move.len()-(move.len()-2)];
	String endPiece = move.chars()[move.len() - (move.len()-2)..];

	println!("{?:} {?:}", startPiece, endPiece);

}