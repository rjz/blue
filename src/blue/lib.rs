#[link(name = "blue",
       vers = "0.0.1",
       url = "")];

#[comment = "Rust HTTP framework"];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod extra;
extern mod http;

pub mod error;
pub mod request;
pub mod response;

// TODO: Replace the hacky HandlerResult with a standard-er type or two.
//       `Result<Either<Request, Response>, Error>`? Could be worse.
pub enum HandlerResult {
    Error(@error::Error),
    Pass(@request::Request),
    Respond(@response::Response)
}

pub trait Handler {
    fn handle(&self, req: @request::Request) -> HandlerResult;
}

// TODO: *Actually* implement filtering / responding / error handling
pub fn run_all (original_request: ~request::GenericRequest, handlers: &[@Handler]) -> HandlerResult {
    handlers.iter().fold(Pass(@*original_request.clone() as @request::Request), |result, f| {
        match result {
            Pass(q)  => f.handle(q),
            Respond(m) => { println("Resp: " + m.to_str()); result }
            Error(e) => {
                println(~"ERROR!" + e.get_message());
                Error(e)
            }
        }
    })
}

