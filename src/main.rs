use schackmotor::*;

fn main() {
    let mut board = Board::new();
    board.populate_board();
    //println!("{:?}", board.table);

    board.table[1][3] = None;

    println!("{:?}", board.table[0][4]);
    // 0, 4 => (1, 5) i människans ögon. (rad, kolumn)

    /* arg: bräda, intialisera vems tur det är */
    let mut notat: AlgebraicNotation = Notation::new(board, schackmotor::_WHITE_PIECE);
    //notat.do_move("Be1xe5");
    /* följer standardnotation, se Wikipedia. */
    assert_eq!(notat.do_move("Bc1d2"), true);
    // returnerar true eller false beroende på utfall
    //notat.do_move("Ke1xd2");

    //let x = schackmotor::Pawn {};
}
