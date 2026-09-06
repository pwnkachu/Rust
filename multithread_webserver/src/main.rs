use std::io::{self, Write, Read, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::os::unix::net::SocketAddr;
use scraper::{Html, Selector};
use std::str::SplitWhitespace;
use std::fmt::{Formatter, Display, Error};

fn get_html(target: &str) -> String {

    let page = match target {
        "/" => "index",
        _ => target.trim_start_matches('/'),
    };

    format!("html/{page}.html");

    // Returns index.html with lazy evaluation for errors
    let response = fs::read_to_string(page).unwrap_or_else(|err| {
        println!("Error in parsing index: {err}");
        "<!DOCTYPE html><html><body>Error 404</body></html>".to_string()
    });

    response
}

// Parses the request's http method and target
fn parse_request_target(
    request: &[String],
) -> Result<(RequestType, &str), ServerError> 
{
    let status_line = request.first().expect("Error: Status Line is absent");

    let mut parts: SplitWhitespace<'_> = status_line.split_whitespace();

    let method:&str  = parts.next().ok_or("Missing HTTP parameter")?;
    let target:&str  = parts.next().ok_or("Missing path")?;
    let version:&str  = parts.next().ok_or("Missing version")?;

    let request_type: RequestType = match method {
        "GET" if method.contains("GET") => RequestType::GET,
        "POST" if method.contains("POST") => RequestType::POST,
        "HEAD" if method.contains("HEAD") => RequestType::HEAD,
        "PUT" if method.contains("PUT") => RequestType::PUT,
        "PATCH" if method.contains("PATCH") => RequestType::PATCH,
        _ => {
            return Err(ServerError::InvalidRequest(
            "Invalid Http Method".to_string(),
            ));
        }
    };

    Ok((request_type, target))
}

// Uses a buffer to read from TcpStream
// Results into a set of lines that are the lines from the request or into an error
fn get_request(stream: &TcpStream) -> Result<Vec<String>, ServerError>{
    
    let buf_reader = BufReader::new(stream.clone());
    let mut http_request = Vec::new();
    
    for line in buf_reader.lines(){
        let result_line = line?;
        if result_line.is_empty(){
            break;
        }
        http_request.push(result_line);
    }
    
    println!("Parsed Request: {http_request:#?}");
    Ok(http_request)
}


fn parse_request(stream: &TcpStream) -> Result<String, ServerError>{
    let http_request:Vec<String>  = get_request(stream)?;
    let (method,target) = parse_request_target(&http_request)?;
    let html = get_html(target);
    return Ok(html);
}

// Handles the request of the client, by parsing the request and crafting the response
// TODO must implement it with a thread pool
fn handle_client(mut stream: TcpStream) -> Result<(),ServerError> {

    print_ip(stream.peer_addr().unwrap());

    let response = parse_request(&stream);
    // TODO craft_response();

    stream.write(response.unwrap().as_bytes());

    Ok(())
}


fn main() -> Result<(), ServerError> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(stream)?;
    }

    Ok(())
}

// Utility
fn print_ip(addr: std::net::SocketAddr){

    println!(
        "Connection established from client: {}",
        addr.ip()
    );

}

// ENUMS
pub enum RequestType{
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

#[derive(Debug)]
enum ServerError {
    Io(io::Error),
    InvalidRequest(String),
}

impl From<io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::Io(error)
    }
}

impl From<&str> for ServerError {
    fn from(error: &str) -> Self {
        ServerError::InvalidRequest(error.to_string())
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            ServerError::Io(error) => {
                write!(f, "erorr I/O: {error}")
            }
            ServerError::InvalidRequest(error) => {
                write!(f, "Invalid request: {error}")
            }
        }
    }
}

impl std::error::Error for ServerError {}