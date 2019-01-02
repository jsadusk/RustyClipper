#[derive(Fail, Debug)]
pub enum ClipperError {
    #[fail(display = "Unexpected C++ exception in call {}", call)]
    CppException { call : &'static str },
    #[fail(display = "Unexpected C++ exception in call {}: {}", call, msg)]
    CppExceptionStr { call : &'static str, msg : String },
    #[fail(display = "Clipper exception in call {}: {}", call, msg)]
    ClipperException { call : &'static str, msg : String }
}

pub type ClipperResult<T> = Result<T, ClipperError>;
