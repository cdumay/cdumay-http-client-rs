use hyper;
use hyper::error::UriError;
use hyper::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json;
use std::borrow::Borrow;
use std::fmt;
use std::io;
use url::ParseError;
use std::str::Utf8Error;

#[derive(Serialize, Deserialize)]
pub struct HTTPException {
    code: u16,
    message: String,
    #[serde(skip_serializing_if = "serde_json::Map::is_empty")]
    extra: serde_json::Map<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "msg-id")]
    msgid: Option<String>
}

impl HTTPException {
    pub fn new(code: u16, message: Option<String>, extra: Option<serde_json::Map<String, serde_json::Value>>, msgid: Option<String>) -> HTTPException {
        HTTPException {
            code: code,
            message: message.unwrap_or("Internal server error".to_string()),
            extra: extra.unwrap_or(serde_json::Map::new()),
            msgid: msgid
        }
    }
    pub fn from_hyper(code: StatusCode, message: Option<String>, extra: Option<serde_json::Map<String, serde_json::Value>>, msgid: Option<String>) -> HTTPException {
        let msg = message.unwrap_or(String::from(code.canonical_reason().unwrap()));
        HTTPException::new(u16::from(code), Some(msg), extra, msgid)
    }
    pub fn code(&self) -> u16 { self.code }
    pub fn message(&self) -> String { self.message.clone() }
    pub fn extra(&self) -> serde_json::Map<String, serde_json::Value> { self.extra.clone() }
}

impl From<StatusCode> for HTTPException {
    fn from(code: StatusCode) -> HTTPException {
        let message = match code.canonical_reason() {
            Some(msg) => Some(msg.to_string()),
            None => None
        };
        HTTPException::new(u16::from(code), message, None, None)
    }
}

impl From<ParseError> for HTTPException {
    fn from(err: ParseError) -> HTTPException {
        let message = format!("{}", err);
        HTTPException::new(500, Some(message), None, None)
    }
}

impl From<Utf8Error> for HTTPException {
    fn from(err: Utf8Error) -> HTTPException {
        let message = format!("{}", err);
        HTTPException::new(500, Some(message), None, None)
    }
}

impl From<io::Error> for HTTPException {
    fn from(err: io::Error) -> HTTPException {
        let message = format!("{}", err);
        HTTPException::new(500, Some(message), None, None)
    }
}

impl From<hyper::Error> for HTTPException {
    fn from(err: hyper::Error) -> HTTPException {
        let message = format!("{}", err);
        HTTPException::new(500, Some(message), None, None)
    }
}

impl From<UriError> for HTTPException {
    fn from(err: UriError) -> HTTPException {
        let message = format!("{}", err);
        HTTPException::new(500, Some(message), None, None)
    }
}

impl From<serde_json::Error> for HTTPException {
    fn from(err: serde_json::Error) -> HTTPException {
        let message = format!("{}", err);
        HTTPException::new(500, Some(message), None, None)
    }
}


impl fmt::Debug for HTTPException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HttpError<code={}, message='{}'>", self.code, self.message)
    }
}

impl fmt::Display for HTTPException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error {}: {} (extra={})", self.code, self.message, serde_json::to_string(&self.extra).unwrap())
    }
}
