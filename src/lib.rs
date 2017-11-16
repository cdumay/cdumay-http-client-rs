extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

pub mod client;
pub mod exceptions;


#[test]
fn test() {
    use hyper::StatusCode;

    let err = exceptions::HTTPException::from_hyper(StatusCode::Created, Some("toto".to_string()), None, Some("plopp".to_string()));
    println!("{}", err);
}

#[test]
fn test_cli() {
    use client::RESTClient;
    use client::RestRequest;
    use serde_json;

    #[derive(Serialize, Deserialize, Debug)]
    struct Post {
        id: u16,
        title: String,
        #[serde(rename="userId")]
        user_id: u16,
        body: String
    }

    let cli = RESTClient::new("http://jsonplaceholder.typicode.com", None, None);
    match cli.do_request::<Post>(hyper::Method::Get, "/posts/1", None, None, None, None) {
        Ok(Some(jdata)) => {
            let p : Post = serde_json::from_str(&jdata).unwrap();
            println!("{:?}", p);
        },
        Ok(None) => println!("Empty response, no data !"),
        Err(e) => println!("{}", e)
    }
}