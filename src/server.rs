use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader, Write},
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

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    println!("Connection reveiced from: {:?}", stream.peer_addr());
    let buffered_reader = BufReader::new(&stream);
    let lines: Vec<_> = buffered_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let http_request = HttpRequest::new(lines)?;
    let response = handle_http_request(http_request)?;
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn handle_http_request(request: HttpRequest) -> Result<String, Box<dyn Error>> {
    let mut status_line = String::new();
    let mut content = String::new();

    if request.query == "/" {
        status_line.push_str("HTTP/1.1 200 OK");
        content = fs::read_to_string("index.html")?;
    } else {
        let file = request.query[1..].to_string();
        if fs::exists(&file)? {
            status_line.push_str("HTTP/1.1 200 OK");
            content = fs::read_to_string(file)?;
        } else {
            status_line.push_str("HTTP/1.1 404 NOT FOUND");
            content = fs::read_to_string("404.html")?;
        }
    }

    let content_length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n {content}");
    Ok(response)
}
