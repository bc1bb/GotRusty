use std::net::{IpAddr, Ipv4Addr};

// -***-
// Server struct
// ---
// Holds basic configuration of our server,
// Can be created using Server::new(addr: &str, port: u16),
// Can get addr or port using Server::get_addr(self) or Server::get_port(self),
// Can set addr or port using Server::set_addr(self, addr: &str) or Server::set_port(self, port: u16).
// -***-

#[derive(Clone, Copy)]
pub struct Server {
    addr: IpAddr,
    port: u16, // int
}

impl Server {
    pub fn new(addr: &str, port: u16) -> Server {
        return Server {
            addr: parse_addr(addr),
            port,
        }
    }

    pub fn get_addr(&self) -> IpAddr { return self.addr }

    pub fn get_port(&self) -> u16 { return self.port }

    pub fn set_addr(&mut self, addr: &str) { self.addr = parse_addr(addr) }

    pub fn set_port(&mut self, port: u16) { self.port = port }
}

// This function is used to parse IPv4 &str into IpAddr::V4,
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

// -***-
// Request struct
// ---
// Holds a request sent by a client,
// Can be created using Request::new().
// -***-

pub struct Request {
    // HTTP HEADERS
    pub command: String,    // GET / HTTP/1.0
    pub host: String,       // Host: 127.0.0.1
    pub user_agent: String, // User-Agent: [whatever]

    // We don't need to read more headers than this,
    // Request.command has to have "HTTP/" and User-Agent (see gr_conn_handler::handler())
}

impl Request {
    pub fn new() -> Request {
        return Request {
            command: "".to_string(),
            host: "Host: ".to_string(),
            user_agent: "User-Agent: ".to_string()
        }
    }
}

// -***-
// Response struct
// ---
// Holds a response that will be sent to a client,
// Can be created using Response::new(r_status: &str, r_content_type: &str, r_content: &str), r_* does not need header names,
// Can be turned into [String; 5] using Response::iter(),
// Can create basic 400 and 404 responses using Response::bad_request() and Response::not_found().
// -***-

pub struct Response {
    // HTTP HEADERS
    status: String,         // HTTP/1.0 200 OK
    server: String,         // Server: {Server.name}
    content_type: String,   // Content-Type: text/html
    content_length: String, // Content-Length: {content.len()}

    // ACTUAL CONTENT
    content: String,           // <title>Got Rusty!</title>
}

impl Response {
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

    // This function allows to iterate through a Response (see gr_conn_handler::sender())
    pub fn iter(self) -> [String; 5] {
        return [self.status, self.server, self.content_type, self.content_length, self.content]
    }

    pub fn bad_request() -> Response {
        return Response::new("400 Bad Request",
                             "text/html",
                             "<h1>Bad Request</h1>")
    }

    pub fn not_found() -> Response {
        return Response::new("404 Not Found",
                             "text/html",
                             "<h1>Not Found</h1>")
    }
}