//! # GotRusty File Handler
//! This file holds all file related stuff required in GR.

use std::env::current_dir;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use crate::gr_structs::{Error, File};

/// # File Reader
/// This function handles a `PathBuf` (as given by client) and returns the content in a `String`,
///
/// If given path is a dir (`PathBuf::is_dir()`), it will try to read path + "index.html",
pub fn get_file(path: PathBuf) -> Result<File, Error> {
    let abs = get_absolute_path(path);

    // use index.html in case user's request is a folder
    if abs.is_dir() {
        //return get_file(abs.join("index.html"))
        let file_name = "index.html";
        let content = read_file(abs.join("index.html").into_boxed_path())?;

        let file = File::new(file_name.to_string(), content);

        return Ok(file)
    }

    let file_name = abs.file_name().unwrap().to_str().unwrap();
    let content = read_file(abs.clone())?;

    let file = File::new(file_name.to_string(), content);

    return Ok(file)
}

fn read_file(path: Box<Path>) -> Result<String, Error> {
    if path.exists() {
        // TODO: handle non UTF-8 data (aka: most non text files) = use vec instead of String
        return Ok(read_to_string(path).unwrap())
    } else {
        return Err(Error::FileNotFound)
    }
}

/// Return complete path of cwd + req.command.path
fn get_absolute_path(path: PathBuf) -> Box<Path> {
    // TODO: allow use of non-cwd base folders
    // TODO: probably make this less hacky...
    let cwd = current_dir().unwrap().to_str().unwrap().to_string();

    // Request path
    let r_path = path.to_str().unwrap().to_string();

    // Complete path
    let c_path = PathBuf::from(cwd.clone() + &*r_path);

    return c_path.into_boxed_path();
}