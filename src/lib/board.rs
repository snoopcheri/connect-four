use std::fmt::{Display, Formatter, Result};

use super::bitboard::BitBoard;
use super::player::Player;
use super::player::Player::*;


pub struct Board {
    eques_pieces: BitBoard,
    knott_pieces: BitBoard,
    side_to_mode: Player,
}


impl Default for Board {
    fn default() -> Self {
        Board {
            eques_pieces: BitBoard::default(),
            knott_pieces: BitBoard::default(),
            side_to_mode: Eques,
        }
    }
}


impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        let mut str = String::new();

        for y in (0..6).rev() {
            str.push_str(&(y + 1).to_string());

            for x in 0..7 {
                str.push(' ');

                let position = (x * 8) + y;

                match self.player_at(position) {
                    Some(player) => str.push_str(&player.to_string()),
                    None => str.push('.'),
                }
            }

            str.push('\n');
        }

        str.push_str("  a b c d e f g\n");

        write!(formatter, "{}\n  {} to move\n", str, self.side_to_mode)
    }
}


impl Board {
    pub fn player_at(&self, position: usize) -> Option<Player> {
        if self.eques_pieces.is_bit_set(position) {
            Some(Eques)
        } else if self.knott_pieces.is_bit_set(position) {
            Some(Knott)
        } else {
            None
        }
    }

    pub fn set_player_at(&mut self, player: Player, position: usize) {
        match player {
            Player::Eques => self.eques_pieces.set_bit(position),
            Player::Knott => self.knott_pieces.set_bit(position),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_at_returns_eques() {
        // arrange
        let mut board = Board::default();

        // act
        board.set_player_at(Player::Eques, 3*8);

        // assert
        assert_eq!(board.player_at(3*8), Some(Player::Eques));
    }


    #[test]
    fn player_at_returns_knott() {
        // arrange
        let mut board = Board::default();

        // act
        board.set_player_at(Player::Knott, 3*8);

        // assert
        assert_eq!(board.player_at(3*8), Some(Player::Knott));
    }


    #[test]
    fn player_at_returns_none() {
        // arrange
        let board = Board::default();

        // act + assert
        assert_eq!(board.player_at(3*8), None);
    }
}
