use crate::errors::keyword_add_error::KeywordAddError;

#[derive(Debug)]
// Erros que o Filter pode retornar
pub enum FilterError {
    SendError, // Pode dar erro no .send(...)
    KeywordAddError(KeywordAddError), // Ou o KeywordAdd pode retornar erro
}