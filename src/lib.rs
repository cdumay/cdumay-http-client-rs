extern crate serde;
extern crate serde_json;
extern crate hyper;

#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod exceptions;


#[test]
fn test() {
    use hyper::StatusCode;

    let err = exceptions::HTTPException::from_hyper(StatusCode::Created, Some("toto"), None, Some("plopp".to_string()));
    println!("{}", err);
}