#![crate_name = "gotrusty"]
extern crate core;

mod gr_conn_handler;
mod gr_file_handler;
mod structs;

use crate::gr_conn_handler::handler;
use std::io::Result;
use std::net::{SocketAddr, TcpListener};
use std::thread::spawn;
use crate::structs::server::Server;

/// # Main Function
/// Creates a `Server`, `SocketAddr`, `TcpListener` (binds it),
///
/// Creates a thread that sends incoming `TcpStream` to `gr_conn_handler::handler()`.
fn main() -> Result<()> {
    // Create a Socket
    let socket = SocketAddr::new(Server::get_addr(), Server::get_port());

    // Bind and listen
    let listener = TcpListener::bind(socket)?;

    // Handle and close connections
    for stream in listener.incoming() {
        spawn(|| {
            handler(stream.unwrap());
        });
    }

    return Ok(());
}
