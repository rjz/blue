#[link(name = "blue",
       vers = "0.0.1",
       url = "")];

#[comment = "Rust HTTP framework"];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod extra;
extern mod http;

use http::server::{ ResponseWriter };
use http::status;

use request::{ Request };
use response::{ Response };

pub mod request;
pub mod response;

pub enum FilterResult<Q, S> {
    Pass(Q),
    Send(S),
    Fail(~str)
}

impl<T: Request, U: Request, V: Response, F: Filter<T, U, V>> FilterResult<T, V> {
    pub fn using (self, f: F) -> FilterResult<U, V> {
        match self {
            Pass(req) => f.filter(req),
            Send(res) => Send(res),
            Fail(s)   => Fail(s)
        }
    }
}

pub trait Filter<ReqIn: Request, ReqOut: Request, ResOut: Response> {
    fn filter(&self, req: ReqIn) -> FilterResult<ReqOut, ResOut>;
}

pub fn handle<P: Response>(httpReq: &http::server::Request) -> FilterResult<request::SimpleRequest, P> {
    Pass(request::generic(~httpReq.clone()))
}

pub fn respond<Q: Request, S: Response> (r: FilterResult<Q, S>, httpRes: &mut ResponseWriter) {

    let (s, b, t) = match r {
        Send(res) => (res.get_status(), res.to_str(), res.content_type()),
        Pass(_) => (500, ~"Miss.", (~"text", ~"plain")),
        Fail(s) => (500, s.to_str(), (~"text", ~"plain"))
    };

    let (type_, subtype) = t;
    let mt = http::headers::content_type::MediaType { type_: type_, subtype: subtype, parameters: ~[] };

    httpRes.status = status::Ok; // Fix this...
    httpRes.write_content_auto(mt, b)
}

