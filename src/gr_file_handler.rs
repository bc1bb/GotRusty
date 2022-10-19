use std::env::current_dir;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

// TODO: handle NotFound/404
// TODO: change mime type of response
pub fn get_file(path: PathBuf) -> String {
    let abs = get_absolute_path(path);

    // use index.html in case user's request is a folder
    if abs.is_dir() {
        return read_file(abs.join("index.html").into_boxed_path());
    }

    return read_file(abs);
}

fn read_file(path: Box<Path>) -> String {
    println!("{}", path.to_str().unwrap().to_string());
    return read_to_string(path).unwrap().to_string()
}

// Return complete path of cwd + req.command.path
fn get_absolute_path(path: PathBuf) -> Box<Path> {
    // TODO: allow use of non-cwd base folders
    // TODO: probably make this less hacky...
    let cwd = current_dir().unwrap().to_str().unwrap().to_string();

    // Request path
    let r_path = path.to_str().unwrap().to_string();

    // Complete path
    let c_path = PathBuf::from(cwd.clone() + &*r_path);

    // Simple but probably useless security check, if client manages to get out of cwd:
    if ! c_path.to_str().unwrap().starts_with(cwd.clone().as_str()) {
        return PathBuf::from(cwd.clone()).into_boxed_path();
    }

    return c_path.into_boxed_path();
}