use crate::errors::keyword_add_error::KeywordAddError;

#[derive(Debug)]
pub enum FilterError {
    SendError,
    KeywordAddError(KeywordAddError),
}