use schackmotor::*;

fn main() {
    let mut board = Board::new();
    board.populate_board();
    //println!("{:?}", board.table);

    /*let pMove:String = "e2e4".to_string();
    let mut end:usize = 2;

    if pMove.len() % 2 == 1 {
        end = 3;
    }

    let startPiece:String = pMove.chars().collect::<String>()[..2].to_string();
    let endPiece:String = pMove.chars().collect::<String>()[end..].to_string();

    println!("{:?} {:?}", startPiece, endPiece); */

    board.table[1][3] = None;

    println!("{:?}", board.table[0][4]);

    let mut notat: AlgebraicNotation = Notation::new(board, schackmotor::_WHITE_PIECE);
    //notat.do_move("Be1xe5");
    notat.do_move("Bc1d2");
    notat.do_move("Ke1xd2");

    //let x = schackmotor::Pawn {};
}
