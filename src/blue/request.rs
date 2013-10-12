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
pub struct DirtyRequest {
    priv original_request: ~::http::server::Request
}

impl DirtyRequest {
    pub fn is_dirty(&self) {
        println("Dirty!")
    }

    pub fn from_request<R: RawRequest> (r: R) -> DirtyRequest {
        DirtyRequest { original_request: r.raw_request().clone() }
    }
}

impl RawRequest for DirtyRequest {
    fn raw_request (&self) -> ~::http::server::Request {
        self.original_request.clone()
    }
}

#[deriving(Clone)]
pub struct SimpleRequest {
    priv original_request: ~::http::server::Request
}

impl SimpleRequest {
    pub fn from_request<R: RawRequest>(r: R) -> SimpleRequest {
        SimpleRequest { original_request: r.raw_request().clone() }
    }
}

impl RawRequest for SimpleRequest {
    fn raw_request (&self) -> ~::http::server::Request {
        self.original_request.clone()
    }
}

pub fn generic (og: ~::http::server::Request) -> SimpleRequest {
    SimpleRequest { original_request: og }
}

