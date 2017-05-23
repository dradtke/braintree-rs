use elementtree;
use hyper;
use std;
use std::convert::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(hyper::Error),
    Api(ApiErrorResponse),
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Error {
        Error::Http(error)
    }
}

impl std::convert::From<Box<std::io::Read>> for Error {
    fn from(xml: Box<std::io::Read>) -> Error {
        let root = elementtree::Element::from_reader(xml).unwrap();
        Error::Api(ApiErrorResponse{
            message: String::from(root.find("message").unwrap().text()),
            raw: root,
        })
    }
}

#[derive(Debug)]
pub struct ApiErrorResponse {
    /// The error message from the response body.
    pub message: String,
    /// The parsed response body returned by the API.
    pub raw: elementtree::Element,
}
