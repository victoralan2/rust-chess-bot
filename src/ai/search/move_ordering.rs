use std::collections::HashMap;
use chess::{Board, ChessMove, Piece};
use crate::ai::evaluation::{calculate_endgame_weight, count_pieces, evaluate_board, get_value, PAWN_VALUE};
use crate::ai::piece_square_tables::peace_square_table_score;

const PROMOTION_MULTIPLIER: i32 = 100;
const CAPTURE_MULTIPLIER: i32 = 50;
const PIECE_SQUARE_MULTIPLIER: i32 = 10;
pub fn sort_moves(board: &Board, moves: &mut Vec<ChessMove>) {
	let mut moves_scores = HashMap::with_capacity(moves.len());

	let to_move_material = count_pieces(board, board.side_to_move());
	let to_move_without_pawns = to_move_material - (board.pieces(Piece::Pawn) & board.color_combined(board.side_to_move())).count() as i32 * PAWN_VALUE;

	let endgame_weight = calculate_endgame_weight(to_move_without_pawns);
	for m in moves.clone() {
		let mut score = 0;
		let target_square = m.get_dest();
		let start_square = m.get_source();
		if let Some(promotion) = m.get_promotion() {
			score+=PROMOTION_MULTIPLIER*get_value(promotion);
		}
		let piece = board.piece_on(start_square).unwrap();
		let piece_value = get_value(piece);
		if let Some(capture_piece) = board.piece_on(target_square) {
			let capture_material_delta = get_value(capture_piece) - piece_value;
			score+=capture_material_delta*CAPTURE_MULTIPLIER;
		}

		// Calculate index of the piece in the square tables
		let to_index = target_square.to_index();
		let from_index = start_square.to_index();

		let to_score = peace_square_table_score(piece, board.side_to_move(), to_index, Some(endgame_weight));
		let from_score = peace_square_table_score(piece, board.side_to_move(), from_index, Some(endgame_weight));
		score += (to_score - from_score) * PIECE_SQUARE_MULTIPLIER;
		moves_scores.insert(m, -score);
	}
	moves.sort_by_cached_key(|x1| moves_scores.get(x1).unwrap());
}

pub fn quiesence_sort_moves(board: &Board, moves: &mut Vec<ChessMove>) {

	let mut moves_scores = HashMap::with_capacity(moves.len());

	let to_move_material = count_pieces(board, board.side_to_move());
	let to_move_without_pawns = to_move_material - (board.pieces(Piece::Pawn) & board.color_combined(board.side_to_move())).count() as i32 * PAWN_VALUE;


	let endgame_weight = calculate_endgame_weight(to_move_without_pawns);

	for m in moves.clone() {
		let mut score = 0;
		let target_square = m.get_dest();
		let start_square = m.get_source();

		let piece = board.piece_on(start_square).unwrap();
		let piece_value = get_value(piece);
		if let Some(capture_piece) = board.piece_on(target_square) {
			let capture_material_delta = get_value(capture_piece) - piece_value/* + (50.0*endgame_weight).round() as i32*/;
			score+=capture_material_delta*CAPTURE_MULTIPLIER*5;
		}

		// Calculate index of the piece in the square tables
		let to_index = target_square.to_index();
		let from_index = start_square.to_index();

		let to_score = peace_square_table_score(piece, board.side_to_move(), to_index, Some(endgame_weight));
		let from_score = peace_square_table_score(piece, board.side_to_move(), from_index, Some(endgame_weight));
		score += (to_score - from_score) * PIECE_SQUARE_MULTIPLIER;

		moves_scores.insert(m, -score);
	}
	moves.sort_by_cached_key(|x1| moves_scores.get(x1).unwrap());// TODO: Check which is better
}