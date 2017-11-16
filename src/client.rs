use futures::{Future, Stream};
use hyper::header::{Headers, ContentType, Accept, qitem};
use hyper::{mime, Method, Client, Error, Uri, Request};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;
use std::time::Duration;
use tokio_core::reactor::{Core, Timeout};
use url::{Url, UrlQuery, ParseError};
use std::str;
use std::fmt::Debug;

pub trait RestRequest {
    fn do_request<T: Serialize>(&self, method: Method, path: &'static str, params: Option<Vec<(&str, &str)>>, data: Option<T>, headers: Option<Headers>, timeout: Option<u64>) -> Result<Option<String>, HTTPException>;
}

pub struct RESTClient {
    server: String,
    timeout: u64,
    headers: Headers
}

impl RESTClient {
    pub fn new(server: &'static str, timeout: Option<u64>, headers: Option<Headers>) -> RESTClient {
        let mut cliheaders: Headers = Headers::new();
        cliheaders.set(ContentType::json());
        cliheaders.set(Accept(vec![qitem(mime::APPLICATION_JSON)]));
        if let Some(h) = headers {
            cliheaders.extend(h.iter());
        }
        RESTClient {
            server: String::from(server),
            timeout: timeout.unwrap_or(10),
            headers: cliheaders
        }
    }
}

use ::exceptions::HTTPException;

impl RestRequest for RESTClient {
    fn do_request<T: Serialize>(&self, method: Method, path: &'static str, params: Option<Vec<(&str, &str)>>, data: Option<T>, headers: Option<Headers>, timeout: Option<u64>) -> Result<Option<String>, HTTPException> {
        let mut url = match params {
            Some(p) => Url::parse_with_params(self.server.as_ref(), &p)?,
            None => Url::parse(self.server.as_ref())?
        };
        url.set_path(path);

        let t = timeout.unwrap_or(10);

        let mut core = Core::new()?;
        let handle = core.handle();
        let client = Client::new(&handle);

        let mut req: Request = Request::new(method.clone(), Uri::from_str(url.as_str())?);
        req.headers_mut().extend(self.headers.iter());
        if let Some(hdrs) = headers {
            req.headers_mut().extend(hdrs.iter());
        }
        req.set_body(match data {
            Some(jdata) => serde_json::to_string(&jdata)?,
            None => String::new()
        });
        let call = client.request(req).and_then(|res| {
            println!("{} {} {}", method, url.as_str(), res.status());
            res.body().concat2()
        });

        //        let timeout = Timeout::new(Duration::from_secs(t), &handle)?;
        //        let work = call.select2(timeout).then(|res| match res {
        //            Ok(Either::A((got, _timeout))) => Ok(got),
        //            Ok(Either::B((_timeout_error, _get))) => {
        //                Err(hyper::Error::Io(io::Error::new(
        //                    io::ErrorKind::TimedOut,
        //                    "Client timed out while connecting",
        //                )))
        //            }
        //            Err(Either::A((get_error, _timeout))) => Err(get_error),
        //            Err(Either::B((timeout_error, _get))) => Err(From::from(timeout_error)),
        //        });

        let result = core.run(call)?;
        match result.is_empty() {
            false => Ok(Some(String::from(str::from_utf8(&result)?))),
            true => Ok(None)
        }
    }
}


impl fmt::Debug for RESTClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RESTClient<server={}>", self.server)
    }
}
