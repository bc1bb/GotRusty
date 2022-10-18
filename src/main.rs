extern crate core;

mod gr_structs;
mod gr_conn_handler;
mod gr_file_handler;

use std::net::{SocketAddr, TcpListener};
use std::io::Result;
use crate::gr_structs::{Request, Response, Server};
use crate::gr_conn_handler::handler;

// TODO for Socket Server:
// create socket (x -> main())
// bind (x -> main())
// listen (x -> main())
// handle (x -> handle())
// send (x -> sender())
// close (x -> main())

// TODO for HTTP Server:
// read request (x -> reader())
// handle request
// read file
// send file

fn main() -> Result<()> {
    // Create a Server
    let my_server = Server::new("127.0.0.1", 1337);

    // Create a Socket
    let socket = SocketAddr::new(my_server.get_addr(), my_server.get_port());

    // Bind and listen
    let listener = TcpListener::bind(socket)?;

    // Handle and close connections
    for stream in listener.incoming() {
        handler(stream?);
    }

    return Ok(())
}

