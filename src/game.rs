#![allow(dead_code)]

use crate::player::Player;
use crate::piece::Piece;
use crate::piece::Sequence;

use rand::seq::SliceRandom;
use rand::thread_rng;

// colors
const RED: u8 = 1;
const BLUE: u8 = 2;
const YELLOW: u8 = 3;
const BLACK: u8 = 4;

const JOKER: u8 = 0;

// sequence categories
const SEQUENCE: u8 = 1; // e.g. 1, 2, 3, 4
const VALUES: u8 = 2; // e.g. 1, 1, 1

pub struct Game {
    players: Vec<Player>,
    pieces: Vec<Piece>,
    board: Vec<Sequence>,
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        // mutable for when jokers are added
        let mut pieces: Vec<Piece> = (1..=4).flat_map(|i: u8| {
            (1..=13).map(move |j: u8| Piece { color: i, value: j })
        }).collect();

        // add jokers
        // let jokers: Vec<Piece> = vec![
        //     Piece { color: JOKER, value: JOKER },
        //     Piece { color: JOKER, value: JOKER },
        // ];
        //
        // pieces.extend(jokers);

        Game {
            players: players,
            pieces: pieces,
            board: vec![],
        }
    }

    pub fn get_random_piece(&mut self) -> Piece {
        self.pieces.remove(0)
    }

    fn initial_deal(&mut self) {
        self.pieces.shuffle(&mut thread_rng());

        for player in &mut self.players {
            for _ in 1..=14 {
                let random_piece = self.pieces.remove(0);
                player.add_piece(random_piece);
            }
        }
    }

    fn player_turn(&mut self, player: &mut Player) {
        player.play_turn(self);
    }
}