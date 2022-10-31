

#[derive(Debug)]
pub enum ParseResult<T> {
    Success(T),
    Error,
    Fatal,
}

impl<T> ParseResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            ParseResult::Success(x) => x,
            ParseResult::Error => panic!("unwrap failed on Error"),
            ParseResult::Fatal => panic!("unwrap failed on Fatal"),
        }
    }
}