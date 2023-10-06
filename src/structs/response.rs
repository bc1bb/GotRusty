use crate::structs::file::File;

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
}