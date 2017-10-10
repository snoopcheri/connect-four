#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;


mod lib;
use lib::prelude::*;


mod errors {
    error_chain! { }
}

use errors::Result;


fn main() {
    let result = run();

    if let Err(ref e) = result {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    let mut board = Board::default();

    board.set_player_at(Player::Eques, 3*8);
    board.set_player_at(Player::Knott, 3*8 + 1);
    board.set_player_at(Player::Eques, 3*8 + 2);

    println!("{}", board);

    Ok(())
}
