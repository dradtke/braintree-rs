use elementtree;
use hyper;
use std;
use std::error::Error as StdError;
use std::convert::From;

pub type Result<T> = std::result::Result<T, Error>;

/// An enum describing an error returned as a result of an API call.
#[derive(Debug)]
pub enum Error {
    /// Braintree's servers reported an error with your request. This usually
    /// means that your authorization was incorrect, you're missing required
    /// fields, or some validation step failed. Consult the error message for
    /// more details about what went wrong.
    Api(ApiErrorResponse),
    /// An HTTP-level error occurred in the underlying client. This usually
    /// means that an error occurred with the raw network call, e.g. no
    /// internet access.
    Http(hyper::Error),
    /// A test operation, such as forcing a transaction into a settlement
    /// status, was attempted in a production environment.
    TestOperationInProduction,
}

impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str(self.description())
        }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Api(ref response) => &response.message,
            Error::Http(ref err) => err.description(),
            Error::TestOperationInProduction => "Operation not allowed in production environment",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        if let Error::Http(ref err) = *self {
            return Some(err)
        }
        None
    }
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
