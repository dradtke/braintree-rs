use elementtree;
use hyper;
use std;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    Api(ApiErrorResponse),
}

impl std::convert::From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Error {
        Error::Http(error)
    }
}

impl std::convert::From<String> for Error {
    fn from(error: String) -> Error {
        let root = elementtree::Element::from_reader(std::io::Cursor::new(&error)).unwrap();
        Error::Api(ApiErrorResponse{
            raw: error,
            message: String::from(root.find("message").unwrap().text()),
        })
    }
}

#[derive(Debug)]
pub struct ApiErrorResponse {
    /// The XML-formatted response body returned by the API.
    pub raw: String,
    /// The error message parsed from the response body.
    pub message: String,
}
