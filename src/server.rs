use std::{
    error::Error,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

use crate::http::HttpRequest;

pub fn run(port: &str) -> Result<(), Box<dyn Error>> {
    let bind_address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(bind_address)?;
    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    Ok(())
}

fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buffered_reader = BufReader::new(stream);
    let lines: Vec<_> = buffered_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request = HttpRequest::new(lines)?;
    println!("Request: {:#?}", http_request);
    Ok(())
}
