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
        println!("{:?}", board.table);
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

        assert_eq!(x.theory_valid_move(&board, false, (2, 1), (3, 1)).ok(), Some(true)); /* single step */
        assert_eq!(q.theory_valid_move(&board, false, (7, 1), (6, 1)).ok(), Some(true)); /* single step */
        assert_eq!(x.theory_valid_move(&board, false, (2, 1), (4, 1)).err(), Some(vec![AdjustPiece { piece: (3, 0), remove_piece: false, increase_movement: 2 }])); /* double step */
        assert_eq!(q.theory_valid_move(&board, false, (7, 1), (5, 1)).err(), Some(vec![AdjustPiece { piece: (4, 0), remove_piece: false, increase_movement: 2 }])); /* double step */

        board.table[(2)][(0)] = Some(Box::new(xa)); /* insert pawn into table. human-wise: (3,1) => (2,0) in table */
        assert_eq!(xa.theory_valid_move(&board, false, (2, 1), (4, 1)).ok(), Some(false)); /* double step */

        /* capture-move when it should not be capturing */
        assert_eq!(x.theory_valid_move(&board, false, (2, 1), (3, 2)).ok(), Some(false));
        assert_eq!(x.theory_valid_move(&board, false, (2, 2), (3, 1)).ok(), Some(false));

        /* capture moves */
        board.table[(2)][(1)] = Some(Box::new(q));
        assert_eq!(xb.theory_valid_move(&board, true, (2, 1), (3, 2)).ok(), Some(true));
        board.table[(2)][(0)] = Some(Box::new(q));
        assert_eq!(xc.theory_valid_move(&board, true, (2, 2), (3, 1)).ok(), Some(true));
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
        assert_eq!(xe.theory_valid_move(&board, true, (4, 4), (3, 3)).ok(), Some(true));

        // white should capture the black
        let xf = Pawn {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        let xg = Pawn {
            color: _BLACK_PIECE,
            has_moved: 2,
        };
        board.table[(4)][(4)] = Some(Box::new(xf));
        board.table[(4)][(5)] = Some(Box::new(xg));
        assert_eq!(xf.theory_valid_move(&board, true, (5, 5), (6, 6)).ok(), Some(true));
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
        assert_eq!(r.theory_valid_move(&board, false, (1, 1), (3, 1)).ok(), Some(false));
        board.table[1][0] = None;
        assert_eq!(r.theory_valid_move(&board, false, (1, 1), (3, 1)).ok(), Some(true));

        /* Sideways */
        assert_eq!(r.theory_valid_move(&board, false, (3, 1), (3, 3)).ok(), Some(true));
        assert_eq!(r.theory_valid_move(&board, false, (3, 3), (3, 2)).ok(), Some(true));

        /* black */
        assert_eq!(ra.theory_valid_move(&board, false, (8, 1), (6, 1)).ok(), Some(false));
        board.table[6][0] = None;
        assert_eq!(ra.theory_valid_move(&board, false, (8, 1), (6, 1)).ok(), Some(true));

        board.table[5][0] = Some(Box::new(ra)); // place black rook to test capture

        /* capture */
        assert_eq!(r.theory_valid_move(&board, true, (1, 1), (7, 1)).ok(), Some(false));
        assert_eq!(r.theory_valid_move(&board, true, (1, 1), (6, 1)).ok(), Some(true)); // try capturing black rook

        /* just illegal behavior */
        assert_eq!(r.theory_valid_move(&board, true, (1, 1), (7, 3)).ok(), Some(false));
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
        assert_eq!(k.theory_valid_move(&board, false, (1, 2), (3, 1)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (1, 2), (3, 3)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (1, 7), (3, 8)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (1, 7), (3, 6)).ok(), Some(true));

        assert_eq!(ka.theory_valid_move(&board, false, (8, 2), (6, 1)).ok(), Some(true));
        assert_eq!(ka.theory_valid_move(&board, false, (8, 2), (6, 3)).ok(), Some(true));
        assert_eq!(ka.theory_valid_move(&board, false, (8, 7), (6, 8)).ok(), Some(true));
        assert_eq!(ka.theory_valid_move(&board, false, (8, 7), (6, 6)).ok(), Some(true));

        /* small capturing test */
        assert_eq!(k.theory_valid_move(&board, true, (5, 4), (7, 3)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, true, (5, 4), (7, 5)).ok(), Some(true));

        /* fix board for free roaming */
        board.table[6][2] = None;
        board.table[6][4] = None;

        /* free roaming knight */
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (7, 3)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (7, 5)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (6, 2)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (6, 6)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (4, 2)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (3, 3)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (3, 5)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (5, 4), (4, 6)).ok(), Some(true));
    }

    #[test]
    fn bishop_basic() {
        let board = Board::new();
        //board.populate_board();

        let b = Bishop {
            color: _WHITE_PIECE,
            has_moved: 0,
        };

        assert_eq!(b.theory_valid_move(&board, false, (1, 3), (3, 5)).ok(), Some(true));
        assert_eq!(b.theory_valid_move(&board, false, (1, 6), (2, 7)).ok(), Some(true));
        assert_eq!(b.theory_valid_move(&board, false, (1, 6), (3, 8)).ok(), Some(true));

        assert_eq!(b.theory_valid_move(&board, false, (8, 3), (7, 2)).ok(), Some(true));
        assert_eq!(b.theory_valid_move(&board, false, (8, 3), (7, 4)).ok(), Some(true));
        assert_eq!(b.theory_valid_move(&board, false, (8, 3), (6, 5)).ok(), Some(true));
        assert_eq!(b.theory_valid_move(&board, false, (8, 6), (7, 7)).ok(), Some(true));
        assert_eq!(b.theory_valid_move(&board, false, (8, 6), (7, 5)).ok(), Some(true));
        assert_eq!(b.theory_valid_move(&board, false, (8, 6), (6, 4)).ok(), Some(true));
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

        assert_eq!(q.theory_valid_move(&board, false, (1, 4), (4, 7)).ok(), Some(true));
        assert_eq!(q.theory_valid_move(&board, false, (1, 4), (5, 4)).ok(), Some(true));
        assert_eq!(q.theory_valid_move(&board, false, (1, 4), (3, 3)).ok(), Some(false)); // try to be a knight
        assert_eq!(q.theory_valid_move(&board, false, (1, 4), (3, 8)).ok(), Some(false)); // just invalid

        assert_eq!(qa.theory_valid_move(&board, false, (8, 4), (5, 1)).ok(), Some(true));
        assert_eq!(qa.theory_valid_move(&board, false, (8, 4), (5, 4)).ok(), Some(true));
        assert_eq!(qa.theory_valid_move(&board, false, (8, 4), (6, 5)).ok(), Some(false)); // try to be a knight
    }

    #[test]
    fn king_basic() {
        let board = Board::new();
        let k = King {
            color: _WHITE_PIECE,
            has_moved: 0,
        };
        assert_eq!(k.theory_valid_move(&board, false, (1, 5), (2, 6)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (1, 5), (2, 4)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (1, 5), (2, 5)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (2, 5), (1, 5)).ok(), Some(true));
        assert_eq!(k.theory_valid_move(&board, false, (2, 5), (1, 4)).ok(), Some(true));

        assert_eq!(k.theory_valid_move(&board, false, (2, 5), (1, 3)).ok(), Some(false));
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
	piece:(isize, isize),
	increase_movement:usize,
	remove_piece: bool
}

pub trait PieceTrait: PieceClone + PieceCommon {
    fn theory_valid_move(
        &self,
        board: &Board,
        capture: bool,
        position: (isize, isize),
        new_position: (isize, isize),
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
        position: (isize, isize),
        new_position: (isize, isize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        let check: bool = false;

        if new_position.0 < position.0 && self.get_color() == _WHITE_PIECE
            || new_position.0 > position.0 && self.get_color() == _BLACK_PIECE
        {
            return Ok(false);
        }

        if !capture {
            /* Single step */
            if board.table[((new_position.0 as usize) - 1)][((new_position.1 as usize) - 1)]
                .is_some()
            {
                return Ok(false);
            }

            if
            /*new_position.0 - position.0 == 1 && new_position.1 - position.1 == 1
            || */
            cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) == 1
                && new_position.1 - position.1 == 0
            {
                return Ok(true);
            }

            /* Double step */
            if cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) == 2
                && new_position.1 - position.1 == 0
                && board.table[((position.0 as usize) - 1)][((position.1 as usize) - 1)]
                    .clone()
                    .unwrap()
                    .movement(0)
                    == 0
            {
                /* Checking the step before */
                if board.table[(new_position.0 as usize) - 2][(new_position.1 as usize) - 1]
                    .is_some()
                {
                    return Ok(false);
                }
				
                return Err(vec![
					AdjustPiece {
						piece: (new_position.0-1, new_position.1-1),
						increase_movement:2,
						remove_piece:false,
					}
				]);
            }
        }

        if capture {
            /* Single step */
            /*if !board.table[((new_position.0 as usize)-1, (new_position.1 as usize)-1)].is_some() {
                return false;
            }*/

            if cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) == 1
                && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1) == 1
                && board.table[((new_position.0 as usize) - 1)][((new_position.1 as usize) - 1)]
                    .is_some()
            /*(new_position.1 - position.1 == 1 || (new_position.1 - position.1) == -1) */
            {
                if board.table[((new_position.0 as usize) - 1)][((new_position.1 as usize) - 1)]
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
            /* Known bug: Will accept two single steps as one double, I think */
            if self.get_color() == _BLACK_PIECE {
				let check_w_black_piece = board.table[(position.0 as usize - 1)][(position.1 as usize - 2)]
				.clone();
                if position.0 == 4
                    && position.1 - 2 >= 0
                    && check_w_black_piece.is_some()
                    && &check_w_black_piece
                        .as_ref().unwrap().get_color()
                        == &_WHITE_PIECE
                    && check_w_black_piece
                        .unwrap()
                        .movement(0)
                        == 2
                    && cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0)
                        == 1
                    && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1)
                        == 1
                {
                    // increase movement
                    return Ok(true);
                }
            } else {
				let check_w_white_piece = board.table[(position.0 as usize - 1)][(position.1 as usize)].clone();
                if position.0 == 5
                    && position.1 - 1 <= 7
                    && check_w_white_piece.is_some()
                    && &check_w_white_piece
                        .as_ref().unwrap()
                        .get_color()
                        == &_BLACK_PIECE
                    && check_w_white_piece
                        .unwrap()
                        .movement(0)
                        == 2
                    && cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0)
                        == 1
                    && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1)
                        == 1
                {
                    //increase movement
                    return Ok(true);
                }
            }
        }

        Ok(check)
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
        position: (isize, isize),
        new_position: (isize, isize),
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
                if board.table[(position.0 - 1) as usize][check as usize].is_some() {
                    return Ok(false);
                }
            }
            return Ok(true);
        }

        if new_position.1 != position.1 {
            return Ok(false);
        }

        if new_position.0 > position.0 {
            for i in position.0..new_position.0 - 1 {
                if (board.table[(i as usize)][((position.1 - 1) as usize)]).is_some() {
                    //println!("{:?} {:?} {:?}", board.table[(i as usize)][((position.1-1) as usize)], i, position.1-1);
                    return Ok(false);
                }
            }
        } else {
            for i in (new_position.0 - 1..position.0 - 1).rev() {
                if (board.table[(i as usize)][((position.1 - 1) as usize)]).is_some() {
                    return Ok(false);
                }
            }
        }

        if capture {
            if board.table[((new_position.0 - 1) as usize)][((new_position.1 - 1) as usize)]
                .is_some()
                && board.table[((new_position.0 - 1) as usize)][((new_position.1 - 1) as usize)]
                    .as_ref()
                    .unwrap()
                    .get_color()
                    != self.get_color()
            {
                return Ok(true);
            }
        } else {
            if !board.table[(new_position.0 - 1) as usize][(new_position.1 - 1) as usize].is_some()
            {
                return Ok(true);
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
        position: (isize, isize),
        new_position: (isize, isize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        if cmp::max(position.1, new_position.1) - cmp::min(position.1, new_position.1) == 1 {
            if cmp::max(position.0, new_position.0) - cmp::min(position.0, new_position.0) == 2 {
                if board.table[(new_position.0 - 1) as usize][(new_position.1 - 1) as usize]
                    .is_some()
                    && !capture
                {
                    return Ok(false);
                }
                return Ok(true);
            }
        }

        if cmp::max(position.1, new_position.1) - cmp::min(position.1, new_position.1) == 2 {
            if cmp::max(position.0, new_position.0) - cmp::min(position.0, new_position.0) == 1 {
                if board.table[(new_position.0 - 1) as usize][(new_position.1 - 1) as usize]
                    .is_some()
                    && !capture
                {
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
        position: (isize, isize),
        new_position: (isize, isize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        if board.table[(new_position.0 - 1) as usize][(new_position.1 - 1) as usize].is_some()
            && !capture
            || (!board.table[(new_position.0 - 1) as usize][(new_position.1 - 1) as usize]
                .is_some()
                && capture)
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
                    let check2 = if new_position.0 > position.0 {
                        check = position.0 - 1 + i;
                        if new_position.1 > position.1 {
                            position.1 - 1 + i
                        } else {
                            position.1 - 1 - i
                        }
                    } else {
                        check = position.0 - 1 - i;
                        if new_position.1 > position.1 {
                            position.1 - 1 + i
                        } else {
                            position.1 - 1 - i
                        }
                    };
                    if board.table[check as usize][check2 as usize].is_some() {
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
        position: (isize, isize),
        new_position: (isize, isize),
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

        if r.theory_valid_move(&board, capture, position, new_position).ok() == Some(true)
            || b.theory_valid_move(&board, capture, position, new_position).ok() == Some(true)
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
        position: (isize, isize),
        new_position: (isize, isize),
    ) -> Result<bool, Vec<AdjustPiece>> {
        if cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) <= 1
            && cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1) <= 1
        /*|| (cmp::max(new_position.0, position.0) - cmp::min(new_position.0, position.0) == 0 &&
        cmp::max(new_position.1, position.1) - cmp::min(new_position.1, position.1) == 1)*/
        {
            if board.table[(new_position.0 - 1) as usize][(new_position.1 - 1) as usize].is_some()
                && !capture
            {
                return Ok(false);
            }
            return Ok(true);
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
    fn find_piece(&self, piece: char, rank: usize, file: usize) -> Vec<FoundPiece>;
}

pub struct AlgebraicNotation {
    base_notation: char,
    board: Board,
    turn: usize,
}

#[derive(Debug)]
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

    fn find_piece(&self, piece: char, rank: usize, file: usize) -> Vec<FoundPiece> {
        /* Will currently silently fail if the piece to be moved is not at turn colorwise */
        //let mut piece:Piece;
        let mut matches: Vec<FoundPiece> = vec![];
        let mut i: usize = 0;

        if rank != 0 && file == 0 {
            let pieces = &self.board.table[rank - 1];
            for p in pieces {
                //println!("p: {:?}", p);
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
            if self.board.table[(rank - 1)][(file - 1)]
                .as_ref()
                .unwrap()
                .get_identity()
                == piece.to_string()
                && self.board.table[(rank - 1)][(file - 1)]
                    .as_ref()
                    .unwrap()
                    .get_color()
                    == self.turn
            {
                matches.push(FoundPiece {
                    piece: piece,
                    position: (rank - 1, file - 1),
                });
            }
        }

        //self.board.table[(rank, file)].as_ref().unwrap().clone()
        matches
    }

    fn do_move(&mut self, p_move: &str) -> bool {
        let mut p_move_chars: Vec<char> = p_move.chars().collect();

        let lookup_piece: char;
        let mut rank: usize = 0;
        let mut file: usize = 0;
        let mut capture: bool = false;
        let mut before: Vec<char> = vec![];
        let mut after: Vec<char> = vec![];

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

        //println!("{:?} {:?}", before, after);

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

        //println!("{:?} {:?} {:?}", file, rank, before);
        let start = self.find_piece(lookup_piece, rank, file);
        //println!("{:?}", start);
        if start.len() == 0 {
            //println!("Not found");
            return false;
        }

        rank = 0;
        file = 0;

        for c in after.iter() {
            if c.is_alphabetic() {
                //println!("{:?}", before);
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

        if rank == 0 || file == 0 {
            println!("Insufficient");
            return false;
        }

        // Is this valid?
        let mut move_occured: bool = false;
        for fp in start {
            if move_occured {
                break;
            }
			let test_move = self.board.table[fp.position.0][fp.position.1]
			.as_ref()
			.unwrap().theory_valid_move(
				&self.board,
				capture,
				((fp.position.0 + 1) as isize, (fp.position.1 + 1) as isize),
				(rank as isize, file as isize),
			);

			if test_move.as_ref().is_ok() == true && test_move.as_ref().ok() != Some(&false)
			|| test_move.as_ref().is_err() == true
            {
                //println!("yes");
                self.board.table[rank - 1][file - 1] = Some(
                    self.board.table[fp.position.0][fp.position.1]
                        .as_ref()
                        .unwrap()
                        .clone(),
                );
				self.board.table[fp.position.0][fp.position.1] = None;
				if test_move.is_err() {
					let correct_board = test_move.err().unwrap();
					for p in correct_board {
						if p.increase_movement >= 1 {
							self.board.table[p.piece.0 as usize][p.piece.1 as usize].as_mut().unwrap().movement(p.increase_movement);
						}
                        if p.remove_piece {
                            self.board.table[p.piece.0 as usize][p.piece.1 as usize] = None;
                        }
					}
				}
                move_occured = true;
            }
        }

        if !move_occured {
            return false;
        }

        //println!("{:?}", self.board.table);

        //println!("{:?}", self.board.table[0][2].as_ref().unwrap().theory_valid_move(&self.board, false, (1, 3), (3, 5)) );
        //println!("{:?} {:?}", file, rank);

        if self.turn == _WHITE_PIECE {
            self.turn = _BLACK_PIECE;
        } else {
            self.turn = _WHITE_PIECE;
        }

        true
    }
}
