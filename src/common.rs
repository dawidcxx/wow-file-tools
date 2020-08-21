use std::error::Error;

pub(crate) type R<T> = Result<T, Box<dyn Error>>;

pub fn err<T>(reason: String) -> R<T> {
    Err(reason.into())
}