use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use crate::{Request, Response};
use crate::gr_structs::Command;

pub fn handler(stream: TcpStream) {
    // Give stream to our request reader
    // which is gonna return a Request
    let req = reader(stream.try_clone().unwrap());

    // if non-HTTP command or no User-Agent, return 400 Bad Request
    if req.user_agent.is_empty() {
        sender(stream.try_clone().unwrap(), Response::bad_request());

        return
    }

    // Create response element
    let res = Response::new("200 OK",
                            "text/html",
                            "<h1>Got Rusty!</h1>");

    // send response we just created
    sender(stream.try_clone().unwrap(), res);
}

fn reader(mut stream: TcpStream) -> Request {
    let mut req = Request::new();
    let buf_reader = BufReader::new(&mut stream);

    // magic line that transforms the BufReader into a Map so we can iterate through
    let request: Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    // Iterate through the map element
    for line in request {
        if line.contains("HTTP/") {
            req.command = Command::new(line.as_str());
        } else if line.starts_with("Host:") {
            req.host = line.to_string();
        } else if line.starts_with("User-Agent:") {
            req.user_agent = line.to_string();
        }
    }

    return req // return is not mandatory but I find it more readable
}

fn sender(mut stream: TcpStream, to_send: Response) {
    for mut i in to_send.iter() {
        i += "\n";

        // Add a newline element on Content-Length because its the last header
        // TODO: make this less hacky
        if i.starts_with("Content-Length:") {
            i += "\n";
        }

        // finally send
        stream.write(i.as_ref()).unwrap();
    }
}