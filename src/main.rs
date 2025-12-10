use std::{env, panic};

mod http;
mod server;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: ./http-server <port>");
    }
    let port = &args[1];
    if let Err(err) = server::run(port) {
        eprintln!("An error occured: {}", err);
    }
}
