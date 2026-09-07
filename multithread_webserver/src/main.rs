use std::io::{self, Write, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::{fs, str};
use std::str::SplitWhitespace;
use std::fmt::{Formatter, Display, Error};
use std::thread;


// Used to get the request html page from a request
// Target is the parsed page from the request
// Return the string that is the requested html file
fn get_html(target: &str) -> Result<(String, HttpStatus), ServerError> {

    let page = match target {
        "/" => "index",
        _ => target.trim_start_matches('/'),
    };

    let path = format!("html/{page}.html");

    // Returns index.html with lazy evaluation for errors
    let file:Result<String, io::Error>  = fs::read_to_string(path);

    let body = match file {
        Ok(html_page) => html_page,
        Err(err) =>  {
            return Err(ServerError::InvalidRequest("Error: {err}".to_string()));
        }
    };

    Ok((body,HttpStatus::Ok))
}

// Parses the request's http method and target
fn parse_status_line(
    request: &[String],
) -> Result<(RequestType, &str), ServerError> 
{
    let status_line = request.first().expect("Error: Status Line is absent");

    let mut parts: SplitWhitespace<'_> = status_line.split_whitespace();

    let method:&str  = parts.next().ok_or("Missing HTTP parameter")?;
    let target:&str  = parts.next().ok_or("Missing path")?;

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

// Return the response linked to a request.
// If any error appears, a ServerError is returned
fn parse_request(stream: &TcpStream) -> Result<(String,HttpStatus), ServerError>{
    // Parse the request from the TcpStream
    let http_request:Vec<String>  = get_request(stream)?;

    // TODO We assume GET as default, must implement other features
    // Used to obtain the page that has been requested
    let (_,target) = parse_status_line(&http_request)?;
    // Obtains the requested html page
    let (html,response_status) = get_html(target)?;

    Ok((html,response_status))
}


// Craft the response in order to visualize the requested html page
fn craft_response(stream: &mut TcpStream, body: String, status: HttpStatus) -> Result<(), ServerError>{

    let length = body.as_bytes().iter().count();

    // Build the response headers
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n{body}"
    );

    let bytes = stream.write(response.as_bytes());

    match bytes {
        Ok(size) => println!("Written {size} bytes as body"),
        Err(e) => {
            return Err(ServerError::InvalidRequest(
            "Error during stream response write: {e}".to_string(),
            ));
        }
    };
    Ok(())
}

// Handles the request of the client, by parsing the request and crafting the response
// TODO must implement it with a thread pool
fn handle_client(mut stream: TcpStream) -> Result<(),ServerError> {

    print_ip(stream.peer_addr().unwrap());

    let response = parse_request(&stream);

    let (response_html, status) = match response{
        Ok(html) => html,
        Err(e) => {
            handle_error(&mut stream);
            panic!("Server error lead to crash {e}")
        }
    };

    match craft_response(&mut stream, response_html, status){
        Ok(html) => html,
        Err(e) => panic!("Response into an error: {e}")
    };

    stream.shutdown(Shutdown::Write).expect("shutdown call failed");

    Ok(())
}

fn handle_error(stream: &mut TcpStream) {
    // Todo return 404
    let body = "</head>
            <body>
                <div class=\"error-container\">
                    <div class=\"error-code\">404</div>
                    <h1>Not found</h1>
                </div>
            </body>
            </html>";
    craft_response(stream, body.to_string(), HttpStatus::NotFound);
}


fn main() -> Result<(), ServerError> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for maybe_stream in listener.incoming() {
        match maybe_stream{
            Ok(stream) => {
                thread::spawn(move || 
                    if let Err(err) = handle_client(stream){
                        eprintln!("Error in client handle {err:?}");
                    });
            }
            Err(err) => panic!("Error: {err}")
        };
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

// Represent all Http Status Codes with associated message
#[derive(Copy, Clone)]
pub enum HttpStatus {
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,

    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    Conflict = 409,
    PayloadTooLarge = 413,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
}

impl HttpStatus {
    pub fn code(&self) -> u16 {
        *self as u16
    }

    pub fn text(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NoContent => "No Content",
            
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::SeeOther => "See Other",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::TemporaryRedirect => "Temporary Redirect",
            HttpStatus::PermanentRedirect => "Permanent Redirect",
            
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
        }
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {}", self.code(), self.text())
    }
}