/// # Error
/// Holds errors to avoid panic!ing.
#[derive(Debug)]
pub enum Error {
    FileNotFound,
    BadRequest,
}