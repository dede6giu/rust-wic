use crate::errors::reqwic_error::ReqWICError;
use crate::errors::filter_error::FilterError;

#[derive(Debug)]
pub enum SendkeysError {
    SendError,
    FilterError(FilterError),
    ReqWICError(ReqWICError),
}