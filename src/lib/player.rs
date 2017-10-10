use std::fmt::{Display, Formatter, Result};


#[derive(Debug, Eq, PartialEq)]
pub enum Player {
    Eques,
    Knott,
}


impl Display for Player {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match *self {
            Player::Eques => write!(formatter, "x"),
            Player::Knott => write!(formatter, "o"),
        }
    }
}
