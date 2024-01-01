use chess::{Color, Piece};
use crate::mathutils::linear_interpolation;

const PAWN_SQUARE_TABLE: [i32; 64] =
	[
		0,  0,  0,  0,  0,  0,  0,  0,
		50, 50, 50, 50, 50, 50, 50, 50,
		10, 10, 20, 30, 30, 20, 10, 10,
		5,  5, 10, 25, 25, 10,  5,  5,
		0,  0,  0, 20, 20,  0,  0,  0,
		5, -5,-10,  0,  0,-10, -5,  5,
		5, 10, 10,-20,-20, 10, 10,  5,
		0,  0,  0,  0,  0,  0,  0,  0
	];
const KNIGHT_SQUARE_TABLE: [i32; 64] =
	[
		-50,-40,-30,-30,-30,-30,-40,-50,
		-40,-20,  0,  0,  0,  0,-20,-40,
		-30,  0, 10, 15, 15, 10,  0,-30,
		-30,  5, 15, 20, 20, 15,  5,-30,
		-30,  0, 15, 20, 20, 15,  0,-30,
		-30,  5, 10, 15, 15, 10,  5,-30,
		-40,-20,  0,  5,  5,  0,-20,-40,
		-50,-40,-30,-30,-30,-30,-40, -50, // TODO: THIS 50 IS A -50
	];
const BISHOP_SQUARE_TABLE: [i32; 64] =
	[
		-20,-10,-10,-10,-10,-10,-10,-20,
		-10,  0,  0,  0,  0,  0,  0,-10,
		-10,  0,  5, 10, 10,  5,  0,-10,
		-10,  5,  5, 10, 10,  5,  5,-10,
		-10,  0, 10, 10, 10, 10,  0,-10,
		-10, 10, 10, 10, 10, 10, 10,-10,
		-10,  5,  0,  0,  0,  0,  5,-10,
		-20,-10,-10,-10,-10,-10,-10,-20,
	];
const ROOK_SQUARE_TABLE: [i32; 64] =
	[
		0,  0,  0,  0,  0,  0,  0,  0,
		5, 10, 10, 10, 10, 10, 10,  5,
		-5,  0,  0,  0,  0,  0,  0, -5,
		-5,  0,  0,  0,  0,  0,  0, -5,
		-5,  0,  0,  0,  0,  0,  0, -5,
		-5,  0,  0,  0,  0,  0,  0, -5,
		-5,  0,  0,  0,  0,  0,  0, -5,
		0,  0,  0,  5,  5,  0,  0,  0
	];

const QUEEN_SQUARE_TABLE: [i32; 64] =
	[
		-20,-10,-10, -5, -5,-10,-10,-20,
		-10,  0,  0,  0,  0,  0,  0,-10,
		-10,  0,  5,  5,  5,  5,  0,-10,
		-5,  0,  5,  5,  5,  5,  0, -5,
		0,  0,  5,  5,  5,  5,  0, -5,
		-10,  5,  5,  5,  5,  5,  0,-10,
		-10,  0,  5,  0,  0,  0,  0,-10,
		-20,-10,-10, -5, -5,-10,-10,-20
	];
const KING_SQUARE_TABLE: [i32; 64] =
	[
		-30,-40,-40,-50,-50,-40,-40,-30,
		-30,-40,-40,-50,-50,-40,-40,-30,
		-30,-40,-40,-50,-50,-40,-40,-30,
		-30,-40,-40,-50,-50,-40,-40,-30,
		-20,-30,-30,-40,-40,-30,-30,-20,
		-10,-20,-20,-20,-20,-20,-20,-10,
		20, 20,  0,  0,  0,  0, 20, 20,
		20, 30, 10,  0,  0, 10, 30, 20
	];
const KING_SQUARE_TABLE_ENDGAME: [i32; 64] =
	[
		-50,-40,-30,-20,-20,-30,-40,-50,
		-30,-20,-10,  0,  0,-10,-20,-30,
		-30,-10, 20, 30, 30, 20,-10,-30,
		-30,-10, 30, 40, 40, 30,-10,-30,
		-30,-10, 30, 40, 40, 30,-10,-30,
		-30,-10, 20, 30, 30, 20,-10,-30,
		-30,-30,  0,  0,  0,  0,-30,-30,
		-50,-30,-30,-30,-30,-30,-30,-50
	];

const SQUARE_TABLE: [[i32; 64]; 7] = [PAWN_SQUARE_TABLE, KNIGHT_SQUARE_TABLE, BISHOP_SQUARE_TABLE, ROOK_SQUARE_TABLE, QUEEN_SQUARE_TABLE, KING_SQUARE_TABLE, KING_SQUARE_TABLE_ENDGAME];

pub fn peace_square_table_score(piece: Piece, perspective: Color, square: usize, endgame_weight: Option<f32>) -> i32 {
	let mut index = 0;
	if perspective == Color::White {
		index= square
	} else {
		index=63 - square
	};
	match piece {
		Piece::Pawn => {
			(PAWN_SQUARE_TABLE[index])
		}
		Piece::Knight => {
			KNIGHT_SQUARE_TABLE[index]
		}
		Piece::Bishop => {
			BISHOP_SQUARE_TABLE[index]
		}
		Piece::Rook => {
			ROOK_SQUARE_TABLE[index]
		}
		Piece::Queen => {
			QUEEN_SQUARE_TABLE[index]
		}
		Piece::King => {
			if let Some(coeff) = endgame_weight {
				return linear_interpolation(KING_SQUARE_TABLE[index] as f32, KING_SQUARE_TABLE_ENDGAME[index] as f32, coeff).round() as i32;
			}
			KING_SQUARE_TABLE[index]
		}
	}
}