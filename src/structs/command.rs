use std::path::PathBuf;
use crate::structs::error::Error;
use crate::structs::error::Error::BadRequest;

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

    /// Function allowing creation of empty Command.
    pub fn adhoc() -> Command {
        return Command {
            method: "GET".to_string(),
            path: PathBuf::from("/"),
            http_version: 1.0,
        };
    }

    pub fn get_path(self) -> PathBuf {
        return self.path;
    }
}