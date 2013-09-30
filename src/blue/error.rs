pub trait Error {
    fn get_message(&self) -> ~str;
}

// This error isn't useful. Presumably future errors will be at least
// marginally evocative...
#[deriving(Clone)]
pub struct SomeError;

impl Error for SomeError {
    fn get_message(&self) -> ~str { ~"Failed with an error" }
}

