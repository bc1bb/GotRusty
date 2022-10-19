//! # GotRusty Structs
//! This file holds all structs and impl used in the project,
//!
//! Lots of 'dead code' to be found here, getters and setters functions mostly.

use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

/// # Server struct
/// Holds basic configuration of our server,
///
/// Can be created using `Server::new(addr: &str, port: u16)`,
///
/// Has `get_*`, `set_*`.
#[derive(Clone, Copy)]
pub struct Server {
    addr: IpAddr,
    port: u16, // int
}

#[allow(dead_code)]
impl Server {
    /// Turns a `&str` and `u16` into a `Server`.
    pub fn new(addr: &str, port: u16) -> Server {
        return Server {
            addr: Server::parse_addr(addr),
            port,
        }
    }

    /// This function is used to parse IPv4 `&str` into `IpAddr::V4`.
    // TODO: probably a better way to do this ?
    fn parse_addr(addr: &str) -> IpAddr {
        // Split str argument into Vec<&str>
        let split_addr: Vec<&str> = addr.split(".").collect();

        // Use splits to build IpAddr::V4
        return IpAddr::V4(Ipv4Addr::new(split_addr[0].to_string().parse().unwrap(),
                                        split_addr[1].to_string().parse().unwrap(),
                                        split_addr[2].to_string().parse().unwrap(),
                                        split_addr[3].to_string().parse().unwrap()))

        // TODO: idiot-proof
    }

    pub fn get_addr(self) -> IpAddr { return self.addr }
    pub fn get_port(self) -> u16 { return self.port }

    pub fn set_addr(&mut self, addr: &str) { self.addr = Server::parse_addr(addr) }
    pub fn set_port(&mut self, port: u16) { self.port = port }
}

/// # Request struct
/// Holds a request sent by a client,
///
/// Can be created using `Request::new()`, which will return basic `Request`,
///
/// Has `get_*`, `set_*`.

#[derive(Clone)]
pub struct Request {
    // HTTP HEADERS
    command: Command,   // GET / HTTP/1.0
    host: String,       // Host: 127.0.0.1
    user_agent: String, // User-Agent: [whatever]

    // We don't need to read more headers than this,
    // Request.command has to have "HTTP/" and User-Agent (see gr_conn_handler::handler())
}

#[allow(dead_code)]
impl Request {
    /// Creates a basic `Request` that can be edited with `set_*`.
    pub fn new() -> Request {
        return Request {
            command: Command::adhoc(),
            host: "Host: ".to_string(),
            user_agent: "User-Agent: ".to_string()
        }
    }

    pub fn get_command(self) -> Command { return self.command }
    pub fn get_host(self) -> String { return self.host }
    pub fn get_user_agent(self) -> String { return self.user_agent }

    pub fn set_command(&mut self, command: Command) { self.command = command }
    pub fn set_host(&mut self, host: String) { self.host = host }
    pub fn set_user_agent(&mut self, user_agent: String) { self.user_agent = user_agent }
}

/// # Command struct
/// Holds a command (first line of HTTP request) sent by a client,
///
/// Can be created using `Command::new(line: &str)`, where line is the complete first line sent by client,
///
/// Has `get_*`, `set_*`.

#[derive(Clone)]
#[allow(dead_code)]
pub struct Command {
    method: String, // GET
    path: PathBuf, // /
    http_version: f32 // HTTP/1.0
}

#[allow(dead_code)]
impl Command {
    /// Turn the first line of an HTTP request into a `Command`,
    ///
    /// If `line` does not contain "HTTP/", an ad-hoc Command will be returned.
    pub fn new(line: &str) -> Command {
        // If invalid input, return adhoc Command
        if ! line.contains("HTTP/") {
            return Command::adhoc()
        }

        let splits: Vec<&str> = line.split(" ").collect();

        return Command {
            method: splits[0].to_string(),
            path: PathBuf::from(splits[1].to_string()),
            http_version: 1.0
        }
    }

    /// Private function allowing creation of empty Command.
    fn adhoc() -> Command {
        return Command {
            method: "GET".to_string(),
            path: PathBuf::from("/"),
            http_version: 1.0
        }
    }

    pub fn get_method(self) -> String { return self.method }
    pub fn get_path(self) -> PathBuf { return self.path }
    pub fn get_http_version(&self) -> f32 { return self.http_version }

    pub fn set_method(&mut self, method: String) { self.method = method }
    pub fn set_path(&mut self, path: PathBuf) { self.path = path }
    pub fn set_http_version(&mut self, http_version: f32) { self.http_version = http_version }
}

/// # Response struct
/// Holds a response that will be sent to a client,
///
/// Can be created using `Response::new(r_status: &str, r_content_type: &str, r_content: &str)`, `r_*` does not need header names,
///
/// Has `get_*`, `set_*`.

pub struct Response {
    // HTTP HEADERS
    status: String,         // HTTP/1.0 200 OK
    server: String,         // Server: {Server.name}
    content_type: String,   // Content-Type: text/html
    content_length: String, // Content-Length: {content.len()}

    // ACTUAL CONTENT
    content: String,           // <title>Got Rusty!</title>
}

#[allow(dead_code)]
impl Response {
    /// Creates a `Response`, `r_*` does not require Headers names.
    pub fn new(r_status: &str, r_content_type: &str, r_content: &str) -> Response {
        // Content-Length requires size in bytes, str::len returns usize (bytes)
        let r_content_length = r_content.len();

        return Response {
            status: "HTTP/1.0 ".to_string() + r_status,
            server: "Server: GotRusty/0.1".to_string(),
            content_type: "Content-Type: ".to_string() + r_content_type,
            content_length: "Content-Length: ".to_string() + r_content_length.to_string().as_str(),

            content: r_content.to_string()
        }
    }

    /// Turn a `Response` into `[String; 5]` (allowing for loops).
    pub fn iter(self) -> [String; 5] {
        return [self.status, self.server, self.content_type, self.content_length, self.content]
    }

    /// Return a basic 400 Bad Request.
    pub fn bad_request() -> Response {
        return Response::new("400 Bad Request",
                             "text/html",
                             "<h1>Bad Request</h1>")
    }

    /// Return a basic 404 Not Found.
    pub fn not_found() -> Response {
        return Response::new("404 Not Found",
                             "text/html",
                             "<h1>Not Found</h1>")
    }

    pub fn get_status(self) -> String { return self.status }
    pub fn get_server(self) -> String { return self.server }
    pub fn get_content_type(self) -> String { return self.content_type }
    pub fn get_content_length(self) -> String { return self.content_length }
    pub fn get_content(self) -> String { return self.content }

    pub fn set_status(&mut self, status: String) { self.status = status }
    pub fn set_content_type(&mut self, content_type: String) { self.content_type = content_type }
    pub fn set_content_length(&mut self, content_length: String) { self.content_length = content_length }
    pub fn set_content(&mut self, content: String) { self.content = content }
}