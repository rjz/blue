extern mod extra;

use extra::json::Json;

pub trait Response : ToStr {
    fn get_status(&self) -> int;
    fn content_type(&self) -> (~str, ~str);
}

// Bleep/bloop. Everything it lacks in general utility will be made up in expedience.
pub struct SimpleResponse {
    status: int,
    body: ~str
}

impl Response for SimpleResponse {
    fn get_status (&self) -> int {
        self.status
    }

    fn content_type (&self) -> (~str, ~str) {
        (~"text", ~"plain")
    }
}

impl ToStr for SimpleResponse {
    fn to_str (&self) -> ~str {
        self.body.clone()
    }
}

// A JSON Response
pub struct JsonResponse {
    status: int,
    body: ~Json
}

impl Response for JsonResponse {
    fn get_status (&self) -> int {
        self.status
    }

    fn content_type (&self) -> (~str, ~str) {
        (~"application", ~"json")
    }
}

impl ToStr for JsonResponse {
    fn to_str (&self) -> ~str {
      self.body.to_pretty_str()
    }
}

