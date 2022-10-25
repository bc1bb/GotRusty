#![crate_name = "gotrusty"]
extern crate core;

mod gr_structs;
mod gr_conn_handler;
mod gr_file_handler;

use std::net::{SocketAddr, TcpListener};
use std::io::Result;
use std::thread;
use crate::gr_structs::{Request, Response, Server};
use crate::gr_conn_handler::handler;

// TODO: config file

/// # Main Function
/// Creates a `Server`, `SocketAddr`, `TcpListener` (binds it),
///
/// Creates a thread that sends incoming `TcpStream` to `gr_conn_handler::handler()`.
fn main() -> Result<()> {
    // Create a Server
    let my_server = Server::new("127.0.0.1", 1337);

    // Create a Socket
    let socket = SocketAddr::new(my_server.get_addr(), my_server.get_port());

    // Bind and listen
    let listener = TcpListener::bind(socket)?;

    // Handle and close connections
    for stream in listener.incoming() {
        thread::spawn(|| { handler(stream.unwrap()); });
    }

    return Ok(())
}

