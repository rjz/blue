extern mod extra;

use extra::json::Json;

pub trait Response : ToStr {
    fn to_bytes(&self) -> ~[u8];
}

// Bleep/bloop. Everything it lacks in general utility will be made up in expedience.
pub struct GenericResponse {
    status: int,
    body: ~str
}

impl ToStr for GenericResponse {
    fn to_str(&self) -> ~str {
        self.status.to_str() + ": " + self.body
    }
}

impl Response for GenericResponse {
    fn to_bytes(&self) -> ~[u8] {
        self.body.as_bytes().to_owned()
    }
}

// A JSON Response
pub struct JsonResponse {
    status: int,
    body: ~Json
}

impl ToStr for JsonResponse {
    fn to_str(&self) -> ~str {
      self.body.to_pretty_str()
    }
}

impl Response for JsonResponse {
    fn to_bytes(&self) -> ~[u8] {
        let s = self.body.to_pretty_str().clone();
        s.as_bytes().to_owned()
    }
}

