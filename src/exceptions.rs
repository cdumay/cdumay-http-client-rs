use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;
use hyper::StatusCode;

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
    pub fn new(code: u16, message: Option<&'static str>, extra: Option<serde_json::Map<String, serde_json::Value>>, msgid: Option<String>) -> HTTPException {
        HTTPException {
            code: code,
            message: message.unwrap_or("Internal server error").to_string(),
            extra: extra.unwrap_or(serde_json::Map::new()),
            msgid: msgid
        }
    }
    pub fn from_hyper(code: StatusCode, message: Option<&'static str>, extra: Option<serde_json::Map<String, serde_json::Value>>, msgid: Option<String>) -> HTTPException {
        let msg = match message {
            None => code.canonical_reason(),
            Some(_) => message,
        };
        HTTPException::new(u16::from(code), msg, extra, msgid)
    }
}

impl From<StatusCode> for HTTPException {
    fn from(code: StatusCode) -> HTTPException {
        HTTPException::new(u16::from(code), code.canonical_reason(), None, None)
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
