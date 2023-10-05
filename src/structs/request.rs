use crate::structs::command::Command;

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