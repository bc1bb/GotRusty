//! # GotRusty Structs
//! This file holds all structs and impl used in the project,
//!
//! Lots of 'dead code' to be found here, getters and setters functions mostly.

use crate::gr_structs::Error::BadRequest;
use std::env::current_dir;
use std::fs::read;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

/// # Error
/// Holds errors to avoid panic!ing.
#[derive(Debug)]
pub enum Error {
    FileNotFound,
    BadRequest,
}

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
        };
    }

    /// This function is used to parse IPv4 `&str` into `IpAddr::V4`.
    // TODO: probably a better way to do this ?
    fn parse_addr(addr: &str) -> IpAddr {
        // Split str argument into Vec<&str>
        let split_addr: Vec<&str> = addr.split(".").collect();

        // Stupido checks
        let mut panic = false;
        if split_addr.clone().len() > 4 {
            panic = true;
        };
        for i in split_addr.clone() {
            let j = i.parse::<u32>();

            if j.is_err() {
                panic = true
            }
            if j.unwrap() > 254 {
                panic = true;
            };
        }

        if panic {
            panic!("Given IP address for server seems invalid.");
        };

        // Use splits to build IpAddr::V4
        return IpAddr::V4(Ipv4Addr::new(
            split_addr[0].to_string().parse().unwrap(),
            split_addr[1].to_string().parse().unwrap(),
            split_addr[2].to_string().parse().unwrap(),
            split_addr[3].to_string().parse().unwrap(),
        ));
    }

    pub fn get_addr(self) -> IpAddr {
        return self.addr;
    }
    pub fn get_port(self) -> u16 {
        return self.port;
    }

    pub fn set_addr(&mut self, addr: &str) {
        return self.addr = Server::parse_addr(addr);
    }
    pub fn set_port(&mut self, port: u16) {
        return self.port = port;
    }
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
    command: Command, // GET / HTTP/1.0
    host: String,     // Host: 127.0.0.1
    user_agent: String, // User-Agent: [whatever]

                      // We don't need to read more headers than this,
                      // Request.command has to have "HTTP/" and a User-Agent
}

#[allow(dead_code)]
impl Request {
    /// Creates a basic `Request` that can be edited with `set_*`.
    pub fn new() -> Request {
        return Request {
            command: Command::adhoc(),
            host: "Host: ".to_string(),
            user_agent: "User-Agent: ".to_string(),
        };
    }

    pub fn get_command(self) -> Command {
        return self.command;
    }
    pub fn get_host(self) -> String {
        return self.host;
    }
    pub fn get_user_agent(self) -> String {
        return self.user_agent;
    }

    pub fn set_command(&mut self, command: Command) {
        return self.command = command;
    }
    pub fn set_host(&mut self, host: String) {
        return self.host = host;
    }
    pub fn set_user_agent(&mut self, user_agent: String) {
        return self.user_agent = user_agent;
    }
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
    method: String,    // GET
    path: PathBuf,     // /
    http_version: f32, // HTTP/1.0
}

#[allow(dead_code)]
impl Command {
    /// Turn the first line of an HTTP request into a `Command`,
    ///
    /// If `line` does not contain "HTTP/" it will return `Error::BadRequest`.
    pub fn new(line: &str) -> Result<Command, Error> {
        // If invalid input, return invalid request
        if !line.contains("HTTP/") {
            return Err(BadRequest);
        };

        let splits: Vec<&str> = line.split(" ").collect();

        return Ok(Command {
            method: splits[0].to_string(),
            path: PathBuf::from(splits[1].to_string()),
            http_version: 1.0,
        });
    }

    /// Private function allowing creation of empty Command.
    fn adhoc() -> Command {
        return Command {
            method: "GET".to_string(),
            path: PathBuf::from("/"),
            http_version: 1.0,
        };
    }

    pub fn get_method(self) -> String {
        return self.method;
    }
    pub fn get_path(self) -> PathBuf {
        return self.path;
    }
    pub fn get_http_version(&self) -> f32 {
        return self.http_version;
    }

    pub fn set_method(&mut self, method: String) {
        return self.method = method;
    }
    pub fn set_path(&mut self, path: PathBuf) {
        return self.path = path;
    }
    pub fn set_http_version(&mut self, http_version: f32) {
        return self.http_version = http_version;
    }
}

/// # Response struct
/// Holds a response that will be sent to a client,
///
/// Can be created using `Response::new(r_status: &str, r_file: File)`,
///
/// Has `get_*`, `set_*`.

pub struct Response {
    // HTTP HEADERS
    status: String,         // HTTP/1.0 200 OK
    server: String,         // Server: {Server.name}
    content_type: String,   // Content-Type: text/html
    content_length: String, // Content-Length: {content.len()}

    // ACTUAL CONTENT
    content: Vec<u8>, // <title>Got Rusty!</title>
}

#[allow(dead_code)]
impl Response {
    /// Creates a `Response` from a status and a `File`
    pub fn new(r_status: &str, r_file: File) -> Response {
        // Content-Length requires size in bytes, str::len returns usize (bytes)
        let r_content_length = r_file.clone().get_content().len();

        return Response {
            status: "HTTP/1.0 ".to_string() + r_status,
            server: "Server: GotRusty/0.1".to_string(),
            content_type: "Content-Type: ".to_string() + r_file.clone().get_mime_type().as_str(),
            content_length: "Content-Length: ".to_string() + r_content_length.to_string().as_str(),

            content: r_file.get_content(),
        };
    }

    /// Turn a `Response` into `[Vec<u8>; 6]` (allowing for loops).
    pub fn iter(self) -> [Vec<u8>; 6] {
        return [
            self.status.as_bytes().to_vec(),
            self.server.as_bytes().to_vec(),
            self.content_type.as_bytes().to_vec(),
            self.content_length.as_bytes().to_vec(),
            "".to_string().as_bytes().to_vec(),
            self.content,
        ];
    }

    /// Return a basic 400 Bad Request.
    pub fn bad_request() -> Response {
        return Response::new("400 Bad Request", File::bad_request());
    }

    /// Return a basic 404 Not Found.
    pub fn not_found() -> Response {
        return Response::new("404 Not Found", File::not_found());
    }

    pub fn get_status(self) -> String {
        return self.status;
    }
    pub fn get_server(self) -> String {
        return self.server;
    }
    pub fn get_content_type(self) -> String {
        return self.content_type;
    }
    pub fn get_content_length(self) -> String {
        return self.content_length;
    }
    pub fn get_content(self) -> Vec<u8> {
        return self.content;
    }

    pub fn set_status(&mut self, status: String) {
        return self.status = status;
    }
    pub fn set_content_type(&mut self, content_type: String) {
        return self.content_type = content_type;
    }

    /// Will automatically define Content-Length as per the `Response.content`.
    pub fn set_content_length(&mut self) {
        let content_length = self.content.len();
        return self.content_length =
            "Content-Length: ".to_string() + content_length.to_string().as_str();
    }

    pub fn set_content(&mut self, content: Vec<u8>) {
        return self.content = content;
    }
}

/// # File struct
/// Holds a file content that will be sent to a client.

#[derive(Clone)]
pub struct File {
    name: String,
    content: Vec<u8>,
    mime_type: String,
}

#[allow(dead_code)]
impl File {
    pub fn new(name: String, content: Vec<u8>) -> File {
        return File {
            mime_type: File::fetch_mime(name.clone()),
            name,
            content,
        };
    }

    fn fetch_mime(name: String) -> String {
        if !name.contains(".") {
            return "text/plain".to_string();
        };

        let ext = name.split(".").last().unwrap();

        let r = match ext {
            // HTML
            "html" => "text/html",
            "htm" => "text/html",
            "xhtml" => "application/xhtml+xml",

            // Important ext's
            "js" => "text/javascript",
            "json" => "application/json",
            "css" => "text/css",
            "xml" => "application/xml",
            "txt" => "text/plain",
            "log" => "text/plain",

            // Images
            "bmp" => "image/bmp",
            "gif" => "image/gif",
            "ico" => "image/vnd.microsoft.icon",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "png" => "image/png",
            "svg" => "image/svg+xml",
            "tif" => "image/tiff",
            "tiff" => "image/tiff",
            "webp" => "image/webp",

            // Fonts
            "otf" => "font/otf",
            "ttf" => "font/ttf",
            "woff" => "font/woff",
            "woff2" => "font/woff2",

            _ => "application/octet-stream",
        };

        return r.to_string();
    }

    fn bad_request() -> File {
        let cwd = current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        return File::new(
            "error.html".to_string(),
            read(PathBuf::from(cwd + "/error/400.html")).unwrap(),
        );
    }

    fn not_found() -> File {
        let cwd = current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        return File::new(
            "error.html".to_string(),
            read(PathBuf::from(cwd + "/error/404.html")).unwrap(),
        );
    }

    pub fn get_name(self) -> String {
        return self.name;
    }
    pub fn get_content(self) -> Vec<u8> {
        return self.content;
    }
    pub fn get_mime_type(self) -> String {
        return self.mime_type;
    }

    pub fn set_name(&mut self, name: String) {
        return self.name = name;
    }
    pub fn set_content(&mut self, content: Vec<u8>) {
        return self.content = content;
    }
    pub fn set_mime_type(mut self) {
        return self.mime_type = File::fetch_mime(self.name);
    }
}
