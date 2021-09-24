use schackmotor::*;

fn main() {
    let mut board = Board::new();
    board.populate_board();
    //println!("{:?}", board.table);

    board.table[1][3] = None;

    println!("{:?}", board.table[0][4]);

    let mut notat: AlgebraicNotation = Notation::new(board, schackmotor::_WHITE_PIECE);
    //notat.do_move("Be1xe5");
    notat.do_move("Bc1d2");
    //notat.do_move("Ke1xd2");

    //let x = schackmotor::Pawn {};
}
