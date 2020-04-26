use std::error::Error;

pub(crate) type R<T> = Result<T, Box<dyn Error>>;