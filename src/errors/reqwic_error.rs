#[derive(Debug)]
// Erros que o ReqWIC pode retornar
pub enum ReqWICError {
    SendError,
    WCMReqWICError
}