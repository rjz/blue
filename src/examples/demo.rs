extern mod extra;
extern mod http;
extern mod blue;

use std::rt::io::net::ip::{ SocketAddr, Ipv4Addr };

use http::server::{ Config, Request, Server, ServerUtil, ResponseWriter };

mod demo {
    use blue::{ Filter, FilterResult, Pass, Send, Fail };
    use blue::request::{ Request, DirtyRequest, SimpleRequest };
    use blue::response::{ SimpleResponse };

    // A simple routing service
    pub struct FizService;

    impl Filter<SimpleRequest, DirtyRequest, SimpleResponse> for FizService {
        fn filter (&self, req: SimpleRequest) -> FilterResult<DirtyRequest, SimpleResponse> {

            // TODO: extract route matching / handling into a more general router
            let method = req.get_method().to_str();
            let path = req.get_uri().to_str();

            let resO = match (method, path) {
                (~"GET", ~"/")    => Some(SimpleResponse { status: 200, body: ~"ok" }),
                (~"GET", ~"/foo") => Some(SimpleResponse { status: 200, body: ~"foo" }),
                _ => None
            };

            let reqOut = DirtyRequest::from_request(req);

            match resO {
                Some(res) => Send(res),
                _ => Pass(reqOut)
            }
        }
    }

    // A logging service
    pub struct LogService;

    impl Filter<SimpleRequest, SimpleRequest, SimpleResponse> for LogService {
        fn filter (&self, req: SimpleRequest) -> FilterResult<SimpleRequest, SimpleResponse> {

            let method = req.get_method().to_str();
            let path = req.get_uri().to_str();

            println(method + " " + path);
            Pass(SimpleRequest::from_request(req))
        }
    }

    // A pass-through service
    pub struct PassService;

    impl Filter<SimpleRequest, SimpleRequest, SimpleResponse> for PassService {
        fn filter (&self, req: SimpleRequest) -> FilterResult<SimpleRequest, SimpleResponse> {
            Pass(SimpleRequest::from_request(req))
        }
    }

    // A naive static file server
    pub struct StaticFilter {
      static_dir: ~str
    }

    impl Filter<DirtyRequest, DirtyRequest, SimpleResponse> for StaticFilter {
        fn filter (&self, req: DirtyRequest) -> FilterResult<DirtyRequest, SimpleResponse> {
            use std::path::Path;
            use std::io;
            use std::os;

            let path = req.get_uri().to_str();
            let p = &Path(self.static_dir + path);

            if (os::path_exists(p) && !os::path_is_dir(p)) {
                match io::read_whole_file_str(p) {
                    Ok(content) => Send(SimpleResponse { status: 200, body: content }),
                    Err(err) => Fail(err)
                }
            }
            else {
                Pass(req.clone())
            }
        }
    }
}

#[deriving(Clone)]
struct DemoServer;

impl Server for DemoServer {

    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 3200 } }
    }

    fn handle_request(&self, httpReq: &Request, httpRes: &mut ResponseWriter) {
        let p = demo::PassService;
        let s = demo::FizService;
        let l = demo::LogService;

        let public = demo::StaticFilter { dir: ~"static" };

        let res = blue::handle(httpReq)
            .using(l)
            .using(p)
            .using(s)
            .using(public);

        blue::respond(res, httpRes)
    }
}

fn main() {
    println("Off and running at :3200");
    DemoServer.serve_forever();
}

