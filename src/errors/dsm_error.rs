use crate::errors::reqwic_error::ReqWICError;
use crate::errors::filter_error::FilterError;

#[derive(Debug)]
pub enum DSMError {
    FilterSendError(FilterError),
    ReqWICSendError(ReqWICError),
}