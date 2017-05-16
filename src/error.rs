use hyper;
use std;

#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    Application(String),
}

impl std::convert::From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Error {
        Error::Http(error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
