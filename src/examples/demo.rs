extern mod extra;
extern mod http;
extern mod blue;

use std::rt::io::net::ip::{ SocketAddr, Ipv4Addr };
use std::rt::io::Writer;

use http::server::{ Config, Server, ServerUtil, Request, ResponseWriter };
use http::status;

use blue::{ Handler, Respond, Pass };

mod demo {

    use extra::json;

    use blue::request::{ Request, generic };
    use blue::response::{ Response, GenericResponse, JsonResponse };
    use blue::{ Handler, Respond, HandlerResult, Pass };

    // GET /
    // Demonstrates sending a JSON response
    fn demo(_: @Request) -> HandlerResult {

        // TODO: even in the middle of a kludgy experiment, this part is
        //       *exceptionally* terrible. We should actually compose the
        //       JSON object to be sent back...
        let res = JsonResponse {
            status: 200,
            body: match json::from_str("{\"hello\":\"world\"}") {
                Ok(r) => ~r,
                _     => ~json::Null
            }
        };

        Respond(@res as @Response)
    }

    // GET /foo
    // Demonstrates sending a generic response
    fn foobar(_: @Request) -> HandlerResult {
        let res = GenericResponse {
            status: 200,
            body: ~"Foobar!"
        };

        Respond(@res as @Response)
    }

    #[deriving(Clone)]
    pub struct Service;

    impl Handler for Service {
        fn handle(&self, r: @Request) -> HandlerResult {
            // TODO: generalize this into some kind of Routing
            match (r.get_method().to_str(), r.get_uri().to_str()) {
                (~"GET", ~"/") => demo(r),
                (~"GET", ~"/foo") => foobar(r),
                _              => Pass(r)
            }
        }
    }

    pub struct LoggerFilter {
        prefix: ~str
    }

    impl Handler for LoggerFilter {
        fn handle(&self, r: @Request) -> HandlerResult {
            // We don't always need to pass the original request through. It
            // may be useful, for instance, to identify a Parsed request or
            // an Authenticated request via explicit types.
            //
            // This is pointless; we could make it less so.
            let dupe = generic(r.raw_request());
            println(self.prefix + " " + r.get_method().to_str() + " " + r.get_uri().to_str());
            Pass(@dupe as @Request)
        }
    }

    pub struct ErrorHandler;

    impl Handler for ErrorHandler {
        fn handle(&self, _: @Request) -> HandlerResult {
            let res = GenericResponse { status: 500, body: ~"Failed" };
            Respond(@res as @Response)
        }
    }
}

#[deriving(Clone)]
struct DemoServer;

impl Server for DemoServer {

    fn get_config(&self) -> Config {
        Config { bind_address: SocketAddr { ip: Ipv4Addr(127, 0, 0, 1), port: 8001 } }
    }

    fn handle_request(&self, r: &Request, w: &mut ResponseWriter) {

        let logFilter: demo::LoggerFilter = demo::LoggerFilter { prefix: ~"Req:" };

        let hs: demo::Service = demo::Service;
        let es: demo::ErrorHandler = demo::ErrorHandler;
        let handlers: &[@blue::Handler] = &[
            @logFilter as @blue::Handler,
            @hs as @blue::Handler,
            @es as @blue::Handler
        ];

        match blue::run_all(~blue::request::generic(~r.clone()), handlers) {
            blue::Pass(_)  => { println("Request wasn't handled appropriately #panick!") },
            blue::Respond(m) => {
                // TODO: add in content-type, at least.
                w.status = status::Ok;
                w.write(m.to_bytes());
            }
            blue::Error(_) => {
                println("error happened.");
            }
        }
    }
}

fn main() {
    println("Off and running at :8001");
    DemoServer.serve_forever();
}

