/*
 * Schackmotor
 * @author Gustaf Haglund <ghaglu@kth.se>
 */

use array2d::Array2D;
use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::fs;

pub const _WHITE_PIECE: usize = 0;
pub const _BLACK_PIECE: usize = 1;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn board_loaded() {
        let mut board = Board::new();
        board.populate_board();
    }

    #[test]
    fn pawn_basic() {
        let mut board = Board::new();
        board.populate_board();
        let x = Pawn {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let q = Pawn {
            color: _BLACK_PIECE,
            has_moved: 0,
        };
        let xa = Pawn {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let xb = Pawn {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let xc = Pawn {
            color: _WHITE_PIECE,
            has_moved: 0,
        };

        assert_eq!(
            x.theory_valid_move(&board, false, (2, 1), (3, 1)).ok(),
            Some(true)
        ); /* single step */
        assert_eq!(
            q.theory_valid_move(&board, false, (7, 1), (6, 1)).ok(),
            Some(true)
        ); /* single step */
        assert_eq!(
            x.theory_valid_move(&board, false, (2, 1), (4, 1)).err(),
            Some(vec![AdjustPiece {
                piece: (3, 0),
                remove_piece: false,
                increase_movement: 2
            }])
        ); /* double step */
        assert_eq!(
            q.theory_valid_move(&board, false, (7, 1), (5, 1)).err(),
            Some(vec![AdjustPiece {
                piece: (4, 0),
                remove_piece: false,
                increase_movement: 2
            }])
        ); /* double step */

        board.table[(2)][(0)] = Some(Box::new(xa)); /* insert pawn into table. human-wise: (3,1) => (2,0) in table */
        assert_eq!(
            xa.theory_valid_move(&board, false, (2, 1), (4, 1)).ok(),
            Some(false)
        ); /* double step */

        /* capture-move when it should not be capturing */
        assert_eq!(
            x.theory_valid_move(&board, false, (2, 1), (3, 2)).ok(),
            Some(false)
        );
        assert_eq!(
            x.theory_valid_move(&board, false, (2, 2), (3, 1)).ok(),
            Some(false)
        );

        /* capture moves */
        board.table[(2)][(1)] = Some(Box::new(q));
        assert_eq!(
            xb.theory_valid_move(&board, true, (2, 1), (3, 2)).ok(),
            Some(true)
        );
        board.table[(2)][(0)] = Some(Box::new(q));
        assert_eq!(
            xc.theory_valid_move(&board, true, (2, 2), (3, 1)).ok(),
            Some(true)
        );
    }

    #[test]
    fn en_passant_basic() {
        let mut board = Board::new();
        let xd = Pawn {
            color: _WHITE_PIECE,
            has_moved: 2,
        };
        let xe = Pawn {
            color: _BLACK_PIECE,
            has_moved: 0,
        };
        board.table[3][2] = Some(Box::new(xd));
        board.table[3][3] = Some(Box::new(xe));
        // black should capture the white
        assert_eq!(
            xe.theory_valid_move(&board, true, (4, 4), (3, 3)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 0,
                remove_piece: true,
                piece: (3, 2)
            }])
        );

        // white should capture the black
        let xf = Pawn {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let xg = Pawn {
            color: _BLACK_PIECE,
            has_moved: 2,
        };
        board.table[4][4] = Some(Box::new(xf));
        board.table[4][5] = Some(Box::new(xg));
        assert_eq!(
            xf.theory_valid_move(&board, true, (5, 5), (6, 6)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 0,
                remove_piece: true,
                piece: (4, 5)
            }])
        );
    }

    #[test]
    fn rook_basic() {
        let mut board = Board::new();
        board.populate_board();

        let r = Rook {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let ra = Rook {
            color: _BLACK_PIECE,
            has_moved: 0,
        };

        /* basic test with default populated board */
        assert_eq!(
            r.theory_valid_move(&board, false, (1, 1), (3, 1)).ok(),
            Some(false)
        );
        board.table[1][0] = None;
        assert_eq!(
            r.theory_valid_move(&board, false, (1, 1), (3, 1)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (2, 0)
            }])
        );

        /* Sideways */
        assert_eq!(
            r.theory_valid_move(&board, false, (3, 1), (3, 3)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (2, 2)
            }])
        );
        assert_eq!(
            r.theory_valid_move(&board, false, (3, 3), (3, 2)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (2, 1)
            }])
        );

        /* black */
        assert_eq!(
            ra.theory_valid_move(&board, false, (8, 1), (6, 1)).ok(),
            Some(false)
        );
        board.table[6][0] = None;
        assert_eq!(
            ra.theory_valid_move(&board, false, (8, 1), (6, 1)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (5, 0)
            }])
        );

        board.table[5][0] = Some(Box::new(ra)); // place black rook to test capture

        /* capture */
        assert_eq!(
            r.theory_valid_move(&board, true, (1, 1), (7, 1)).ok(),
            Some(false)
        );
        assert_eq!(
            r.theory_valid_move(&board, true, (1, 1), (6, 1)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (5, 0)
            }])
        ); // try capturing black rook

        /* just illegal behavior */
        assert_eq!(
            r.theory_valid_move(&board, true, (1, 1), (7, 3)).ok(),
            Some(false)
        );
    }

    #[test]
    fn knight_basic() {
        let mut board = Board::new();
        board.populate_board();

        let k = Knight {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let ka = Knight {
            color: _BLACK_PIECE,
            has_moved: 0,
        };

        /* basic movement */
        assert_eq!(
            k.theory_valid_move(&board, false, (1, 2), (3, 1)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (1, 2), (3, 3)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (1, 7), (3, 8)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (1, 7), (3, 6)).ok(),
            Some(true)
        );

        assert_eq!(
            ka.theory_valid_move(&board, false, (8, 2), (6, 1)).ok(),
            Some(true)
        );
        assert_eq!(
            ka.theory_valid_move(&board, false, (8, 2), (6, 3)).ok(),
            Some(true)
        );
        assert_eq!(
            ka.theory_valid_move(&board, false, (8, 7), (6, 8)).ok(),
            Some(true)
        );
        assert_eq!(
            ka.theory_valid_move(&board, false, (8, 7), (6, 6)).ok(),
            Some(true)
        );

        /* small capturing test */
        assert_eq!(
            k.theory_valid_move(&board, true, (5, 4), (7, 3)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, true, (5, 4), (7, 5)).ok(),
            Some(true)
        );

        /* fix board for free roaming */
        board.table[6][2] = None;
        board.table[6][4] = None;

        /* free roaming knight */
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (7, 3)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (7, 5)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (6, 2)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (6, 6)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (4, 2)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (3, 3)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (3, 5)).ok(),
            Some(true)
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (5, 4), (4, 6)).ok(),
            Some(true)
        );
    }

    #[test]
    fn bishop_basic() {
        let board = Board::new();
        //board.populate_board();

        let b = Bishop {
            color: _WHITE_PIECE,
            has_moved: 0,
        };

        assert_eq!(
            b.theory_valid_move(&board, false, (1, 3), (3, 5)).ok(),
            Some(true)
        );
        assert_eq!(
            b.theory_valid_move(&board, false, (1, 6), (2, 7)).ok(),
            Some(true)
        );
        assert_eq!(
            b.theory_valid_move(&board, false, (1, 6), (3, 8)).ok(),
            Some(true)
        );

        assert_eq!(
            b.theory_valid_move(&board, false, (8, 3), (7, 2)).ok(),
            Some(true)
        );
        assert_eq!(
            b.theory_valid_move(&board, false, (8, 3), (7, 4)).ok(),
            Some(true)
        );
        assert_eq!(
            b.theory_valid_move(&board, false, (8, 3), (6, 5)).ok(),
            Some(true)
        );
        assert_eq!(
            b.theory_valid_move(&board, false, (8, 6), (7, 7)).ok(),
            Some(true)
        );
        assert_eq!(
            b.theory_valid_move(&board, false, (8, 6), (7, 5)).ok(),
            Some(true)
        );
        assert_eq!(
            b.theory_valid_move(&board, false, (8, 6), (6, 4)).ok(),
            Some(true)
        );
    }

    #[test]
    fn queen_basic() {
        let board = Board::new();
        let q = Queen {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let qa = Queen {
            color: _BLACK_PIECE,
            has_moved: 0,
        };

        assert_eq!(
            q.theory_valid_move(&board, false, (1, 4), (4, 7)).ok(),
            Some(true)
        );
        assert_eq!(
            q.theory_valid_move(&board, false, (1, 4), (5, 4)).ok(),
            Some(true)
        );
        assert_eq!(
            q.theory_valid_move(&board, false, (1, 4), (3, 3)).ok(),
            Some(false)
        ); // try to be a knight
        assert_eq!(
            q.theory_valid_move(&board, false, (1, 4), (3, 8)).ok(),
            Some(false)
        ); // just invalid

        assert_eq!(
            qa.theory_valid_move(&board, false, (8, 4), (5, 1)).ok(),
            Some(true)
        );
        assert_eq!(
            qa.theory_valid_move(&board, false, (8, 4), (5, 4)).ok(),
            Some(true)
        );
        assert_eq!(
            qa.theory_valid_move(&board, false, (8, 4), (6, 5)).ok(),
            Some(false)
        ); // try to be a knight
    }

    #[test]
    fn king_basic() {
        let board = Board::new();
        let k = King {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        assert_eq!(
            k.theory_valid_move(&board, false, (1, 5), (2, 6)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (1, 5)
            }])
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (1, 5), (2, 4)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (1, 3)
            }])
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (1, 5), (2, 5)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (1, 4)
            }])
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (2, 5), (1, 5)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (0, 4)
            }])
        );
        assert_eq!(
            k.theory_valid_move(&board, false, (2, 5), (1, 4)).err(),
            Some(vec![AdjustPiece {
                increase_movement: 1,
                remove_piece: false,
                piece: (0, 3)
            }])
        );

        assert_eq!(
            k.theory_valid_move(&board, false, (2, 5), (1, 3)).ok(),
            Some(false)
        );
    }

    #[test]
    fn promotion_test() {
        let mut board = Board::new();
        board.table[6][4] = Some(Box::new(Pawn {
            has_moved: 0,
            color: _WHITE_PIECE,
        }));
        let mut algnot: AlgebraicNotation = Notation::new(board, _WHITE_PIECE);
        assert_eq!(algnot.do_move("e8Q"), true);
        assert_eq!(
            algnot.board.table[7][4].as_ref().unwrap().get_identity(),
            "Q".to_string()
        );
        assert_eq!(algnot.board.table[6][4].as_ref().is_some(), false);
    }

    #[test]
    fn checkmate_threat_test() {
        let mut board = Board::new();
        board.table[2][4] = Some(Box::new(King {
            has_moved: 0,
            color: _WHITE_PIECE,
        }));
        board.table[2][3] = Some(Box::new(Rook {
            has_moved: 0,
            color: _BLACK_PIECE,
        }));
        let mut algnot: AlgebraicNotation = Notation::new(board, _WHITE_PIECE);
        assert_eq!(algnot.do_move("Ke3f3"), false);
        assert_eq!(algnot.do_move("Ke3xd3"), true);
    }

    #[test]
    fn castling_test() {
        let mut board = Board::new();
        let mut board2 = Board::new();
        board.table[0][4] = Some(Box::new(King {
            has_moved: 0,
            color: _WHITE_PIECE,
        }));
        board.table[0][0] = Some(Box::new(Rook {
            has_moved: 0,
            color: _WHITE_PIECE,
        }));
        board2.table[0][4] = board.table[0][4].clone();
        board2.table[0][7] = board.table[0][0].clone();
        let mut algnot_queen: AlgebraicNotation = Notation::new(board, _WHITE_PIECE);
        let mut algnot_king: AlgebraicNotation = Notation::new(board2, _WHITE_PIECE);

        /* Queenside castling */
        assert_eq!(algnot_queen.do_move("0-0-0"), true);
        assert_eq!(
            algnot_queen.board.table[0][2]
                .as_ref()
                .unwrap()
                .get_identity(),
            "K".to_string()
        );
        assert_eq!(
            algnot_queen.board.table[0][3]
                .as_ref()
                .unwrap()
                .get_identity(),
            "R".to_string()
        );

        /* Kingside castling */
        assert_eq!(algnot_king.do_move("0-0"), true);
        assert_eq!(
            algnot_king.board.table[0][6]
                .as_ref()
                .unwrap()
                .get_identity(),
            "K".to_string()
        );
        assert_eq!(
            algnot_king.board.table[0][5]
                .as_ref()
                .unwrap()
                .get_identity(),
            "R".to_string()
        );
    }
}

#[derive(Debug, Clone)]
struct Move {
    before: Vec<u32>,
    after: Vec<u32>,
}

/*struct Position {
    row: u32,
    column: u32
}*/

#[derive(Debug, Clone)]
pub struct Piece {
    //position: Position,
    identity: String,
    color: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Pawn {
    color: usize,
    has_moved: usize,
}
#[derive(Debug, Clone)]
pub struct Rook {
    color: usize,
    has_moved: usize,
}
#[derive(Debug, Clone)]
pub struct Knight {
    color: usize,
    has_moved: usize,
}
#[derive(Debug, Clone)]
pub struct Bishop {
    color: usize,
    has_moved: usize,
}
#[derive(Debug, Clone)]
pub struct Queen {
    color: usize,
    has_moved: usize,
}
#[derive(Debug, Clone)]
pub struct King {
    color: usize,
    has_moved: usize,
}

/*#[derive(Debug, Clone)]
enum Pieces {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
}*/

#[derive(Debug, Clone)]
pub struct Board {
    //pub table: Array2D<Option<Box<dyn PieceTrait>>>,
    pub table: Vec<Vec<Option<Box<dyn PieceTrait>>>>,
    history: Vec<Move>,
    short_pieces: Vec<char>,
    pieces: HashMap<char, Box<dyn PieceTrait>>,
}

pub trait PieceClone {
    fn clone_box(&self) -> Box<dyn PieceTrait>;
}

pub trait PieceCommon {
    fn set_color(&mut self, color: usize);
    fn get_color(&self) -> usize;
    fn movement(&mut self, movement: usize) -> usize;
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdjustPiece {
    piece: (usize, usize),
    increase_movement: usize,
    remove_piece: bool,
}

pub trait PieceTrait: PieceClone + PieceCommon {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (usize, usize),
        new_position: (usize, usize),
    ) -> Result<bool, Vec<AdjustPiece>>;
    fn get_identity(&self) -> String;
}

impl fmt::Debug for dyn PieceTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Piece")
            .field("identity", &self.get_identity())
            .finish()
    }
}

impl<T> PieceClone for T
where
    T: 'static + PieceTrait + Clone,
{
    fn clone_box(&self) -> Box<dyn PieceTrait> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn PieceTrait> {
    fn clone(&self) -> Box<dyn PieceTrait> {
        self.clone_box()
    }
}

macro_rules! impl_piececommon {
	($($t:ty),+ $(,)?) => ($(
		impl PieceCommon for $t {
			fn set_color(&mut self, color: usize) {
				self.color = color;
			}
			fn get_color(&self) -> usize {
				self.color
			}

			fn movement(&mut self, movement: usize) -> usize {
				if movement != 0 {
					self.has_moved += movement;
				}
				self.has_moved
			}
		}
	)+);
}

impl PieceTrait for Pawn {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (usize, usize),
        new_position: (usize, usize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        if new_position.0 < position.0 && self.get_color() == _WHITE_PIECE
            || new_position.0 > position.0 && self.get_color() == _BLACK_PIECE
        {
            return Ok(false);
        }

        if !capture {
            /* Single step */
            if board.table[new_position.0 - 1][new_position.1 - 1].is_some() {
                return Ok(false);
            }

            if cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) == 1
                && new_position.1 - position.1 == 0
            {
                return Ok(true);
            }

            /* Double step */
            if cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) == 2
                && new_position.1 - position.1 == 0
                && board.table[position.0 - 1][position.1 - 1]
                    .clone()
                    .unwrap()
                    .movement(0)
                    == 0
            {
                /* Checking the step before */
                if board.table[new_position.0 - 2][new_position.1 - 1].is_some() {
                    return Ok(false);
                }

                return Err(vec![AdjustPiece {
                    piece: (new_position.0 - 1, new_position.1 - 1),
                    increase_movement: 2,
                    remove_piece: false,
                }]);
            }
        }

        if capture {
            /* Single step */

            if cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) == 1
                && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1) == 1
                && board.table[new_position.0 - 1][new_position.1 - 1].is_some()
            {
                if board.table[(new_position.0) - 1][(new_position.1) - 1]
                    .clone()
                    .unwrap()
                    .get_color()
                    == self.get_color()
                {
                    return Ok(false);
                }
                return Ok(true);
            }

            /* ... Passant? */
            /* Known bug: Will accept two single steps as one double, I think..? */
            if position.0 == 0 {
                return Ok(false);
            }
            if self.get_color() == _BLACK_PIECE {
                if position.1 <= 1 {
                    return Ok(false);
                }
                let check_w_black_piece = board.table[position.0 - 1][position.1 - 2].clone();
                if position.0 == 4
                    && check_w_black_piece.is_some()
                    && &check_w_black_piece.as_ref().unwrap().get_color() == &_WHITE_PIECE
                    && check_w_black_piece.unwrap().movement(0) == 2
                    && cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0)
                        == 1
                    && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1)
                        == 1
                {
                    //return Ok(true);
                    return Err(vec![AdjustPiece {
                        piece: (position.0 - 1, position.1 - 2),
                        increase_movement: 0,
                        remove_piece: true,
                    }]);
                }
            } else {
                let check_w_white_piece = board.table[position.0 - 1][position.1].clone();
                if position.0 == 5
                    && position.1 - 1 <= 7
                    && check_w_white_piece.is_some()
                    && &check_w_white_piece.as_ref().unwrap().get_color() == &_BLACK_PIECE
                    && check_w_white_piece.unwrap().movement(0) == 2
                    && cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0)
                        == 1
                    && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1)
                        == 1
                {
                    //return Ok(true);
                    return Err(vec![AdjustPiece {
                        piece: (position.0 - 1, position.1),
                        increase_movement: 0,
                        remove_piece: true,
                    }]);
                }
            }
        }

        Ok(false)
    }
    fn get_identity(&self) -> String {
        "P".to_string()
    }
}
impl_piececommon!(Pawn);

impl PieceTrait for Rook {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (usize, usize),
        new_position: (usize, usize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        /*if position.1 != new_position.1 || position.0 == new_position.0 {
            return false;
        }*/

        // sideways
        if position.0 == new_position.0 {
            let delta_x =
                cmp::max(position.1, new_position.1) - cmp::min(position.1, new_position.1);

            for i in 1..delta_x {
                let check = if position.1 > new_position.1 {
                    position.1 - 1 - i
                } else {
                    position.1 - 1 + i
                };
                if board.table[position.0 - 1][check].is_some() {
                    return Ok(false);
                }
            }
            //return Ok(true);
            return Err(vec![AdjustPiece {
                piece: (new_position.0 - 1, new_position.1 - 1),
                increase_movement: 1,
                remove_piece: false,
            }]);
        }

        if new_position.1 != position.1 {
            return Ok(false);
        }

        if new_position.0 > position.0 {
            for i in position.0..new_position.0 - 1 {
                if (board.table[i][position.1 - 1]).is_some() {
                    return Ok(false);
                }
            }
        } else {
            for i in (new_position.0 - 1..position.0 - 1).rev() {
                if (board.table[i][position.1 - 1]).is_some() {
                    return Ok(false);
                }
            }
        }

        if capture {
            if board.table[new_position.0 - 1][new_position.1 - 1].is_some()
                && board.table[new_position.0 - 1][new_position.1 - 1]
                    .as_ref()
                    .unwrap()
                    .get_color()
                    != self.get_color()
            {
                //return Ok(true);
                return Err(vec![AdjustPiece {
                    piece: (new_position.0 - 1, new_position.1 - 1),
                    increase_movement: 1,
                    remove_piece: false,
                }]);
            }
        } else {
            if !board.table[new_position.0 - 1][new_position.1 - 1].is_some() {
                //return Ok(true);
                return Err(vec![AdjustPiece {
                    piece: (new_position.0 - 1, new_position.1 - 1),
                    increase_movement: 1,
                    remove_piece: false,
                }]);
            }
        }
        Ok(false)
    }
    fn get_identity(&self) -> String {
        "R".to_string()
    }
}

impl_piececommon!(Rook);

impl PieceTrait for Knight {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (usize, usize),
        new_position: (usize, usize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        if cmp::max(position.1, new_position.1) - cmp::min(position.1, new_position.1) == 1 {
            if cmp::max(position.0, new_position.0) - cmp::min(position.0, new_position.0) == 2 {
                if board.table[new_position.0 - 1][new_position.1 - 1].is_some() && !capture {
                    return Ok(false);
                }
                return Ok(true);
            }
        }

        if cmp::max(position.1, new_position.1) - cmp::min(position.1, new_position.1) == 2 {
            if cmp::max(position.0, new_position.0) - cmp::min(position.0, new_position.0) == 1 {
                if board.table[new_position.0 - 1][new_position.1 - 1].is_some() && !capture {
                    return Ok(false);
                }
                return Ok(true);
            }
        }

        Ok(false)
    }
    fn get_identity(&self) -> String {
        "K".to_string()
    }
}
impl_piececommon!(Knight);

impl PieceTrait for Bishop {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (usize, usize),
        new_position: (usize, usize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        if board.table[new_position.0 - 1][new_position.1 - 1].is_some() && !capture
            || (!board.table[new_position.0 - 1][new_position.1 - 1].is_some() && capture)
        {
            return Ok(false);
        }

        if new_position.0 > position.0 || new_position.0 < position.0 {
            let delta_x =
                cmp::max(position.1, new_position.1) - cmp::min(position.1, new_position.1);
            let delta_y =
                cmp::max(position.0, new_position.0) - cmp::min(position.0, new_position.0);
            if delta_x == delta_y {
                /* Validation, is this correct? TODO FIXME */

                for i in 1..delta_y {
                    let check;
                    let check2 = {
                        if new_position.0 > position.0 {
                            check = position.0 - 1 + i;
                        } else {
                            check = position.0 - 1 - i;
                        }
                        if new_position.1 > position.1 {
                            position.1 - 1 + i
                        } else {
                            position.1 - 1 - i
                        }
                    };
                    if board.table[check][check2].is_some() {
                        return Ok(false);
                    }
                }
                return Ok(true);
            }
        }

        Ok(false)
    }
    fn get_identity(&self) -> String {
        "B".to_string()
    }
}

impl_piececommon!(Bishop);

impl PieceTrait for Queen {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (usize, usize),
        new_position: (usize, usize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        /* Well, a glorified bishop and rook, and I'm lazy... */
        let r = Rook {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let b = Bishop {
            color: _WHITE_PIECE,
            has_moved: 0,
        };

        if r.theory_valid_move(&board, capture, position, new_position)
            .is_err()
            || b.theory_valid_move(&board, capture, position, new_position)
                .ok()
                == Some(true)
        {
            return Ok(true);
        }

        Ok(false)
    }
    fn get_identity(&self) -> String {
        "Q".to_string()
    }
}
impl_piececommon!(Queen);

impl PieceTrait for King {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (usize, usize),
        new_position: (usize, usize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        if cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) <= 1
            && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1) <= 1
        {
            if board.table[new_position.0 - 1][new_position.1 - 1].is_some() && !capture {
                return Ok(false);
            }
            //return Ok(true);
            return Err(vec![AdjustPiece {
                piece: (new_position.0 - 1, new_position.1 - 1),
                increase_movement: 1,
                remove_piece: false,
            }]);
        }

        Ok(false)
    }
    fn get_identity(&self) -> String {
        "K".to_string()
    }
}
impl_piececommon!(King);

impl Board {
    pub fn new() -> Board {
        let mut pieces = HashMap::<char, Box<dyn PieceTrait>>::new();
        pieces.insert(
            'R',
            Box::new(Rook {
                color: _WHITE_PIECE,
                has_moved: 0,
            }),
        );
        pieces.insert(
            'N',
            Box::new(Knight {
                color: _WHITE_PIECE,
                has_moved: 0,
            }),
        );
        pieces.insert(
            'B',
            Box::new(Bishop {
                color: _WHITE_PIECE,
                has_moved: 0,
            }),
        );
        pieces.insert(
            'Q',
            Box::new(Queen {
                color: _WHITE_PIECE,
                has_moved: 0,
            }),
        );
        pieces.insert(
            'K',
            Box::new(King {
                color: _WHITE_PIECE,
                has_moved: 0,
            }),
        );
        pieces.insert(
            'P',
            Box::new(Pawn {
                color: _WHITE_PIECE,
                has_moved: 0,
            }),
        );

        Board {
            table: Array2D::filled_with(None, 8, 8).as_rows(), //vec![vec![None]],
            history: vec![],
            short_pieces: vec!['R', 'K', 'B', 'K', 'Q', 'P'],
            pieces: pieces,
        }
    }

    pub fn populate_board(&mut self) {
        let mut row: usize = 0;
        let mut column: usize = 0;
        let mut iter = 0;

        let contents =
            fs::read_to_string("board_definition").expect("Something went wrong reading the file");
        for c in contents.replace("\n", "").chars() {
            if iter == 8 {
                row += 1;
                column = 0;
                iter = 0;
            }

            self.table[row][column] = Some(self.pieces.get(&c).unwrap().clone());

            let mut localpiece;
            localpiece = self.pieces.get(&c).unwrap().clone();
            localpiece.set_color(_BLACK_PIECE);
            self.table[7 - row][column] = Some(localpiece);

            column += 1;
            iter += 1;
        }
    }
}

pub trait Notation {
    fn new(board: Board, turn: usize) -> Self;
    fn do_move(&mut self, p_move: &str) -> bool;
    fn find_piece(
        &self,
        board: &Board,
        piece: char,
        rank: usize,
        file: usize,
        color: usize,
    ) -> Vec<FoundPiece>;
    fn check_king_threat(&mut self, board: Board) -> Result<bool, FoundPiece>;
}

pub struct AlgebraicNotation {
    base_notation: char,
    board: Board,
    turn: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct FoundPiece {
    piece: char,
    position: (usize, usize),
}

impl Notation for AlgebraicNotation {
    fn new(board: Board, turn: usize) -> AlgebraicNotation {
        AlgebraicNotation {
            base_notation: 'a',
            board: board,
            turn: turn,
        }
    }

    fn find_piece(
        &self,
        board: &Board,
        piece: char,
        rank: usize,
        file: usize,
        color: usize,
    ) -> Vec<FoundPiece> {
        /* Will currently silently fail if the piece to be moved is not at turn colorwise */
        let mut matches: Vec<FoundPiece> = vec![];
        let mut i: usize = 0;

        if rank == 0 && file == 0 {
            let pieces = &board.table;
            let mut ir = 0;
            i = 0;
            for r in pieces {
                for p in r {
                    if p.as_ref().is_none() {
                        i += 1;
                        continue;
                    }
                    if piece != 0 as char && p.as_ref().unwrap().get_identity() != piece.to_string()
                    {
                        i += 1;
                        continue;
                    }
                    if p.as_ref().unwrap().get_color() == color {
                        matches.push(FoundPiece {
                            piece: (p.as_ref().unwrap().get_identity())
                                .chars()
                                .collect::<Vec<char>>()[0],
                            position: (ir + 1, i + 1),
                        });
                    }
                    i += 1;
                }
                ir += 1;
                i = 0;
            }
        }

        if rank != 0 && file == 0 {
            let pieces = &self.board.table[rank - 1];
            for p in pieces {
                if p.as_ref().unwrap().get_identity() == piece.to_string()
                    && p.as_ref().unwrap().get_color() == self.turn
                {
                    println!("Found {:?} at rank {:?}", piece, rank);
                    matches.push(FoundPiece {
                        piece: piece,
                        position: (rank - 1, i),
                    });
                }
                i += 1;
            }
        }

        if rank != 0 && file != 0 {
            if self.board.table[rank - 1][file - 1].is_some() {
                let p_piece = self.board.table[rank - 1][file - 1].as_ref().unwrap();
                if p_piece.get_identity() == piece.to_string() && p_piece.get_color() == self.turn {
                    matches.push(FoundPiece {
                        piece: piece,
                        position: (rank - 1, file - 1),
                    });
                }
            }
        }

        //self.board.table[(rank, file)].as_ref().unwrap().clone()
        matches
    }

    fn check_king_threat(&mut self, board: Board) -> Result<bool, FoundPiece> {
        let king;
        /* Edge case; e.g. promotion tests... will not impact real sessions */
        /* it could be in some case possible to endanger the king without triggering the code, but then it's a bug */
        let temp = self.find_piece(&board, 'K', 0, 0, self.turn);
        if temp.len() > 0 {
            king = temp[0];
        } else {
            return Ok(false);
        }
        let enemy_pieces = self.find_piece(
            &board,
            0 as char,
            0,
            0,
            if self.turn == _WHITE_PIECE {
                _BLACK_PIECE
            } else {
                _WHITE_PIECE
            },
        );
        for p in enemy_pieces {
            if p.position != king.position {
                /* to account that the king might decide to himself strike down the enemy */
                let test_move = board.table[p.position.0 - 1][p.position.1 - 1]
                    .as_ref()
                    .unwrap()
                    .theory_valid_move(&board, true, p.position, king.position);
                if test_move.as_ref().is_ok() == true && test_move.as_ref().ok() != Some(&false)
                    || test_move.as_ref().is_err() == true
                {
                    println!(
                        "{:?} at ({:?}) threatens king at {:?}",
                        board.table[p.position.0 - 1][p.position.1 - 1]
                            .as_ref()
                            .unwrap()
                            .get_identity(),
                        p.position,
                        king.position
                    );
                    return Err(king);
                }
            }
        }
        Ok(false)
    }

    fn do_move(&mut self, p_move: &str) -> bool {
        let mut p_move_chars: Vec<char> = p_move.chars().collect();

        let lookup_piece: char;
        let mut rank: usize = 0;
        let mut file: usize = 0;
        let mut capture: bool = false;
        let mut before: Vec<char> = vec![];
        let mut after: Vec<char> = vec![];
        let mut promotion: char = 0 as char;
        let mut threatened = false;
        let mut move_occured = false;
        let mut start: Vec<FoundPiece> = vec![];

        let chmthreat = self.check_king_threat(self.board.clone());
        if chmthreat.is_err() {
            threatened = true;
        }

        if p_move.contains('-') {
            /* Castling */
            let mut king_pos = self.find_piece(&self.board, 'K', 0, 0, self.turn)[0].position;
            king_pos = (king_pos.0 - 1, king_pos.1 - 1);
            let king = self.board.table[king_pos.0][king_pos.1].as_mut().unwrap();
            if king.movement(0) > 0 || threatened || !(p_move == "0-0" || p_move == "0-0-0") {
                return false;
            }

            let rook_pos_vec = self.find_piece(
                &self.board,
                'R',
                if self.turn == _WHITE_PIECE { 1 } else { 8 },
                if p_move == "0-0" { 8 } else { 1 },
                self.turn,
            );
            let rook_pos;
            if rook_pos_vec.len() == 1 {
                rook_pos = rook_pos_vec[0].position;
            } else {
                return false;
            }
            //well, different behaviour depending on rank, file.. rook_pos = (rook_pos.0-1, rook_pos.1-1);
            let rook = self.board.table[rook_pos.0][rook_pos.1].as_mut().unwrap();
            if rook.movement(0) > 0 {
                return false;
            }
            // delta x is 3
            for i in 1..3 {
                let offset = if p_move == "0-0" {
                    king_pos.1 + i
                } else {
                    king_pos.1 - i
                };
                if self.board.table[king_pos.0][offset].is_some() {
                    return false;
                }
                let mut tboard = self.board.clone();
                tboard.table[king_pos.0][offset] = tboard.table[king_pos.0][king_pos.1].clone();
                tboard.table[king_pos.0][king_pos.1] = None;
                if self.check_king_threat(tboard).is_err() {
                    return false;
                }
            }
			// well, queen-sidewise does the rook need to pass this...
			if p_move == "0-0-0" {
				for i in 1..4 {
					if self.board.table[rook_pos.0][rook_pos.1+i].is_some() {
						return false;
					}
				}
			}

            let final_offset_king: isize = if p_move == "0-0" { 2 } else { -2 };
            let final_offset_rook: isize = if p_move == "0-0" { -2 } else { 3 };
            self.board.table[king_pos.0][(king_pos.1 as isize + final_offset_king) as usize] =
                self.board.table[king_pos.0][king_pos.1].clone();
            self.board.table[rook_pos.0][(rook_pos.1 as isize + final_offset_rook) as usize] =
                self.board.table[rook_pos.0][rook_pos.1].clone();
            self.board.table[king_pos.0][king_pos.1] = None;
            self.board.table[rook_pos.0][rook_pos.1] = None;

            move_occured = true;
            lookup_piece = 0 as char; // dumb rust compilator
        } else if p_move_chars.len() == 3 {
            /* Promotion */
            before = p_move_chars[..2].to_vec();
            after = before.clone();
            before[1] = ((before[1] as u8) - 1) as char;
            lookup_piece = 'P';
            promotion = p_move_chars[2];
            if promotion == 'P' {
                return false; // disallow promoting to pawn
            }
        } else {
            if p_move_chars[1] == 'x' {
                /* Well, pawn */
                lookup_piece = 'P';
            } else {
                lookup_piece = p_move_chars[0];
                p_move_chars = p_move_chars[1..].to_vec();
            }

            if p_move.contains('x') {
                capture = true;
                let capture_i = p_move.find('x');
                match capture_i {
                    Some(i) => {
                        before = p_move_chars[..i - 1].to_vec();
                        after = p_move_chars[(i)..].to_vec();
                    }
                    None => (),
                };
            } else {
                if p_move_chars.len() == 4 {
                    before = p_move_chars[..2].to_vec();
                    after = p_move_chars[2..].to_vec();
                } else {
                    before = p_move_chars[..1].to_vec();
                    after = p_move_chars[1..].to_vec();
                }
            }
        }

        if before.len() >= 1 {
            for c in before.iter() {
                if c.is_alphabetic() {
                    //println!("{:?}", before);
                    file = *c as usize - self.base_notation as usize + 1;
                } else {
                    match c.to_digit(10) {
                        Some(i) => {
                            rank = i as usize;
                        }
                        None => (),
                    }
                }
            }
        }

        if !move_occured {
            start = self.find_piece(&self.board, lookup_piece, rank, file, 0);
            if start.len() == 0 {
                return false;
            }
        }

        let b_rf = (rank, file);
        rank = 0;
        file = 0;

        for c in after.iter() {
            if c.is_alphabetic() {
                file = *c as usize - self.base_notation as usize + 1;
            /* theory_valid_move accepts human-wise arg., hence +1 */
            } else {
                match c.to_digit(10) {
                    Some(i) => {
                        rank = i as usize;
                    }
                    None => (),
                }
            }
        }

        if (rank == 0 || file == 0) && !move_occured {
            println!("Insufficient");
            return false;
        }

        if threatened {
            if chmthreat.err().unwrap().position != (b_rf.0, b_rf.1) {
                return false;
            }
            // see if the (not yet) validated move would work
            let mut tboard = self.board.clone();
            tboard.table[rank - 1][file - 1] = tboard.table[b_rf.0][b_rf.1].clone();
            tboard.table[b_rf.0][b_rf.1] = None;

            if self.check_king_threat(tboard).is_err() {
                return false;
            }
        }

        // Is this valid?
        for fp in start {
            if move_occured {
                break;
            }
            let test_move = self.board.table[fp.position.0][fp.position.1]
                .as_ref()
                .unwrap()
                .theory_valid_move(
                    &self.board,
                    capture,
                    ((fp.position.0 + 1), (fp.position.1 + 1)),
                    (rank, file),
                );

            if test_move.as_ref().is_ok() == true && test_move.as_ref().ok() != Some(&false)
                || test_move.as_ref().is_err() == true
            {
                if promotion == 0 as char {
                    self.board.table[rank - 1][file - 1] = Some(
                        self.board.table[fp.position.0][fp.position.1]
                            .as_ref()
                            .unwrap()
                            .clone(),
                    );
                } else {
                    self.board.table[rank - 1][file - 1] =
                        Some(self.board.pieces.get(&promotion).unwrap().clone());

                    if self.turn == _BLACK_PIECE {
                        self.board.table[rank - 1][file - 1]
                            .as_mut()
                            .unwrap()
                            .set_color(_BLACK_PIECE);
                    }
                }
                self.board.table[fp.position.0][fp.position.1] = None;
                if test_move.is_err() {
                    let correct_board = test_move.err().unwrap();
                    for p in correct_board {
                        if p.increase_movement >= 1 {
                            self.board.table[p.piece.0][p.piece.1]
                                .as_mut()
                                .unwrap()
                                .movement(p.increase_movement);
                        }
                        if p.remove_piece {
                            self.board.table[p.piece.0][p.piece.1] = None;
                        }
                    }
                }
                move_occured = true;
            }
        }

        if !move_occured {
            return false;
        }

        if self.turn == _WHITE_PIECE {
            self.turn = _BLACK_PIECE;
        } else {
            self.turn = _WHITE_PIECE;
        }

        true
    }
}
