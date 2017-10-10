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
    let mut bb= BitBoard::default();

    bb.set_bit(3*8);
    bb.set_bit(3*8 + 1);
    bb.set_bit(3*8 + 2);
    bb.clear_bit(3*8 + 1);

    println!("bb: #1={}\n{}", bb.count_ones(), bb);

    Ok(())
}
