//! # GotRusty Connection Handler
//! `gr_conn_handler` is dedicated to handling/replying to `TcpStream` using `handler()`, `reader()` and `sender()`.

use crate::gr_file_handler::get_file;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use crate::structs::command::Command;
use crate::structs::error::Error;
use crate::structs::request::Request;
use crate::structs::response::Response;

/// # Request Handler
/// Public Function that is gonna handle a `TcpStream`.
pub fn handler(stream: TcpStream) {
    // Give stream to our request reader
    // which is gonna return a Request

    let file = match reader(stream.try_clone().unwrap()) {
        Ok(req) => get_file(req.get_command().get_path()),
        Err(e) => Err(e),
    };

    // Create response element
    let res = match file {
        Ok(file) => Response::new("200 OK", file),
        Err(Error::FileNotFound) => Response::not_found(),
        Err(Error::BadRequest) => Response::bad_request(),
    };

    // send response we just created
    sender(stream.try_clone().unwrap(), res);
}

/// # Request Reader
/// Private function that is gonna turn `TcpStream` into a `Request` (see `gr_structs`).
fn reader(mut stream: TcpStream) -> Result<Request, Error> {
    let mut req = Request::new();
    let buf_reader = BufReader::new(&mut stream);

    // magic line that transforms the BufReader into a Map so we can iterate through
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Iterate through the map element
    for (i, line) in request.iter().enumerate() {
        if i == 0 {
            if line.contains("HTTP/") {
                req.set_command(Command::new(line.as_str())?);
            } else {
                return Err(Error::BadRequest);
            }
        }

        if line.starts_with("Host:") {
            req.set_host(line.to_string());
        } else if line.starts_with("User-Agent:") {
            req.set_user_agent(line.to_string());
        }
    }

    return Ok(req);
}

/// # Request Sender
/// Private function that is gonna write `Response` (see `gr_structs`) to client.
fn sender(mut stream: TcpStream, to_send: Response) {
    for mut i in to_send.iter() {
        i.push(b'\n');

        // finally send
        stream.write(i.as_ref()).unwrap();
    }
}
