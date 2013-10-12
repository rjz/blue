extern mod extra;
extern mod http;
extern mod blue;

use std::rt::io::net::ip::{ SocketAddr, Ipv4Addr };

use http::server::{ Config, Request, Server, ServerUtil, ResponseWriter };


mod demo {
    use extra::json;
    use extra::treemap::TreeMap;

    use blue::{ Filter, FilterResult, Pass, Send, Fail };
    use blue::request::{ Request, DirtyRequest, SimpleRequest };
    use blue::response::{ SimpleResponse };

    type Route<B> = (~str, ~str, extern fn(~B) -> Option<SimpleResponse>);

    // A simple routing service
    // TODO: generalize route matching, callback parameterization
    pub struct RoutingService {
        // `<R: Request>` would be a much better callback parameter than `int`
        routes: ~[Route<int>]
    }

    impl Filter<SimpleRequest, DirtyRequest, SimpleResponse> for RoutingService {
        fn filter (&self, req: SimpleRequest) -> FilterResult<DirtyRequest, SimpleResponse> {

            let method = req.get_method().to_str();
            let path = req.get_uri().to_str();
            let req = DirtyRequest::from_request(req);

            // Check that method / path match
            let resO = self.routes.iter().skip_while(|&r| {
                match r.clone() {
                    (m, p, _) => (m != method || p != path)
                }
            }).next();

            // Check that the handler actually returns a response
            match resO {
              Some(a) => match a {
                &(_, _, func) => match func(~42) {
                  Some(res) => Send(res),
                  _ => Pass(req)
                }
              },
              _ => Pass(req)
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

    // A naive static file server
    pub struct StaticFilter {
      static_dir: ~str
    }

    impl Filter<DirtyRequest, DirtyRequest, SimpleResponse> for StaticFilter {
        fn filter (&self, req: DirtyRequest) -> FilterResult<DirtyRequest, SimpleResponse> {
            use std::{ io, os };
            use std::path::Path;

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

    // Build a router
    pub fn a_router () -> RoutingService {

        fn json_pair (a: ~str, b: ~str) -> ~json::Json {
            let mut j = TreeMap::new();
            j.insert(a, json::String(b));
            ~json::Object(~j.clone())
        }

        fn a (i: ~int) -> Option<SimpleResponse> {
          println("'a' called with: " + i.to_str());
          Some(SimpleResponse::from_json(200, json_pair(~"hello", ~"world")))
        };

        fn b (i: ~int) -> Option<SimpleResponse> {
          println("'b' called with: " + i.to_str());
          Some(SimpleResponse { body: ~"bar", status: 200 })
        };

        RoutingService { routes: ~[
            (~"GET", ~"/", a),
            (~"GET", ~"/foo", b),
        ]}
    }

}

#[deriving(Clone)]
struct DemoServer;

impl Server for DemoServer {

    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 3200 } }
    }

    fn handle_request(&self, httpReq: &Request, httpRes: &mut ResponseWriter) {

        let l = demo::LogService;

        let public = demo::StaticFilter { static_dir: ~"static" };

        let res = blue::handle(httpReq)
            .using(l)
            .using(demo::a_router())
            .using(public);

        blue::respond(res, httpRes)
    }
}

fn main() {
    println("Off and running at :3200");
    DemoServer.serve_forever();
}

