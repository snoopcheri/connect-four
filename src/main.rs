#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;


mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }
    }
}



use errors::*;
use errors::Result;


mod lib;
use lib::prelude::*;


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
    let bb= BitBoard::new();

    Ok(())
}
