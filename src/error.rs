use std::io;

#[derive(Debug)]
pub enum Error {
    Options(String),
    IO(io::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}
