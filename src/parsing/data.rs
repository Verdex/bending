
#[derive(Debug)]
pub enum ParseResult<T> {
    Success(T),
    Error,
    Fatal,
}