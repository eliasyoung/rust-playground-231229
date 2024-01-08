pub use super::request_method::RequestMethod;
use crate::http::request_method::{self, MethodError};
use std::convert::{From, TryFrom};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

pub struct Request<'a>{
    path: &'a str,
    query_string: Option<&'a str>,
    method: RequestMethod,
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
    type Error = ParseError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(value) {
        //     Ok(request) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),

        // }
        // match str::from_utf8(value).or(Err(ParseError::InvalidEncoding)) {
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        // let request = str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?;

        let request = str::from_utf8(value)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: RequestMethod = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })

    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Error for ParseError {}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}
