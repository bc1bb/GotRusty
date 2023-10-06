/// # File struct
/// Holds a file content that will be sent to a client.
use std::fs::read;
use std::path::PathBuf;
use crate::structs::server::Server;

#[derive(Clone)]
#[allow(dead_code)]
pub struct File {
    name: String,
    content: Vec<u8>,
    mime_type: String,
}

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

        let mime_default = Server::get_mime_default();
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

            _ => &*mime_default,
        };

        return r.to_string();
    }

    pub fn bad_request() -> File {
        return File::new(
            "error.html".to_string(),
            read(PathBuf::from(Server::get_errors_root() + "/400.html")).unwrap(),
        );
    }

    pub fn not_found() -> File {
        return File::new(
            "error.html".to_string(),
            read(PathBuf::from(Server::get_errors_root() + "/404.html")).unwrap(),
        );
    }

    pub fn get_content(self) -> Vec<u8> {
        return self.content;
    }
    pub fn get_mime_type(self) -> String {
        return self.mime_type;
    }
}
