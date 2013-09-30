extern mod http;

use http::server::request::{ RequestUri };
use http::method::{ Method };

pub trait RawRequest: Clone {
    fn raw_request(&self) -> ~::http::server::Request;
}

pub trait Request: Clone + RawRequest {
    fn get_method(&self) -> Method;
    fn get_uri(&self) -> RequestUri;
}

impl<T: RawRequest> Request for T {
    fn get_method(&self) -> Method {
        self.raw_request().method
    }

    fn get_uri(&self) -> RequestUri {
        self.raw_request().request_uri
    }
}

#[deriving(Clone)]
pub struct GenericRequest {
    priv original_request: ~::http::server::Request
}

impl RawRequest for GenericRequest {
    fn raw_request (&self) -> ~::http::server::Request {
        self.original_request.clone()
    }
}

impl RawRequest for ~GenericRequest {
    fn raw_request (&self) -> ~::http::server::Request {
        self.original_request.clone()
    }
}

pub fn generic (og: ~::http::server::Request) -> GenericRequest {
    GenericRequest { original_request: og }
}

