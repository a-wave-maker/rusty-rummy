use crate::piece::Piece;
use crate::game::Game;

pub struct Player {
    pieces: Vec<Piece>,
    started: bool,
    moved: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            pieces: vec![],
            started: false,
            moved: false,
        }
    }

    pub fn add_piece(&mut self, piece: Piece) {
        self.pieces.push(piece);
    }

    pub fn remove_piece(&mut self, index: usize) -> Piece {
        self.pieces.remove(index)
    }

    pub fn play_turn(&mut self, game: &mut Game) {
        let moves = self.find_moves();
        if moves.len() == 0 && !self.started {
            self.add_piece(game.get_random_piece());
        }

        self.moved = false;
    }

    fn find_moves(&mut self) -> Vec<Vec<usize>> {
        self.pieces.sort_by(|a, b| a.value.cmp(&b.value));

        let mut moves: Vec<Vec<usize>> = vec![];
        let mut current_index: usize = 0;

        while current_index < self.pieces.len() {

            let mut potential_value_move: Vec<usize> = vec![current_index];
            let mut looking_for_value_move = true;
            let mut potential_sequence_move: Vec<usize> = vec![current_index];
            let mut looking_for_sequence_move = true;

            let mut i: usize = 1;

            while current_index + i < self.pieces.len() {
                let next_index = current_index + i;
                let next_piece = &self.pieces[next_index];

                if next_piece.value != self.pieces[current_index].value {
                    looking_for_value_move = false;
                }
                if next_piece.value > self.pieces[potential_sequence_move[potential_sequence_move.len() - 1]].value + 1 {
                    looking_for_sequence_move = false;
                }

                // looking for same values different colors
                if looking_for_value_move {
                    
                    let mut has_different_color = true;
                    for &piece_index in &potential_value_move {
                        let piece = &self.pieces[piece_index];
                        if next_piece.color == piece.color {
                            has_different_color = false;
                            break;
                        }
                    }

                    if has_different_color {
                        potential_value_move.push(next_index);
                    }
                }    
                
                if looking_for_sequence_move {
                    let last_piece = &self.pieces[potential_sequence_move[potential_sequence_move.len() - 1]];

                    if next_piece.value == last_piece.value && next_piece.color == last_piece.color {
                        potential_sequence_move.push(next_index);
                    }
                }

                i += 1;
            }

            if Player::check_move(&potential_value_move) {
                moves.push(potential_value_move.clone());
            }
            if Player::check_move(&potential_sequence_move) {
                moves.push(potential_sequence_move.clone());
            }

            potential_value_move.clear();
            potential_sequence_move.clear();

            current_index += 1;

        }

        moves
    }

    fn sort_moves(&mut self, moves: &mut Vec<Vec<usize>>) {
        moves.sort_by(|a, b| {
            let len_cmp = b.len().cmp(&a.len());
            if len_cmp != std::cmp::Ordering::Equal {
                return len_cmp;
            }

            // if lengths are equal, compare sum of values
            let sum_a: u8 = a.iter().map(|piece_index| self.pieces[*piece_index].value).sum();
            let sum_b: u8 = b.iter().map(|piece_index| self.pieces[*piece_index].value).sum();
            sum_b.cmp(&sum_a)
        });
    }

    fn board_manipulations(&mut self, board: &mut Vec<Vec<Piece>>) {
        // adding to existing sequences

        // splitting existing sequences

        // taking from existing sequences

        // rearranging existing sequences
    }

    fn check_move(potential_move: &Vec<usize>) -> bool {
        if potential_move.len() < 3 {
            return false;
        }

        return true
    }

}