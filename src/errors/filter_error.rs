#[derive(Debug)]
// Erros que o Filter pode retornar
pub enum FilterError {
    SendError, // Pode dar erro no .send(...)
    KeywordAddError, // Ou o KeywordAdd pode retornar erro
}