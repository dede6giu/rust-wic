use std::collections::HashSet;

pub fn extract_sentences(text: &str) -> Vec<String> {
    let mut sentence = match text.chars().next() {
        Some(c) => c.to_string(), // Inicializa a frase com a primeira letra do texto, se houver
        None => return vec![], // Se não houver, a função retorna o vetor vazio
    };
    
    let mut l = 1;
    // Percorre o texto até a próxima letra maiúscula
    for letter in text[1..].chars() {
        if !letter.is_uppercase() {
            sentence.push(letter); // E forma a frase
            l += 1;
        } else {
            break;
        }
    };

    let mut sentences = vec![sentence.trim().to_string()];
    sentences.extend(extract_sentences(&text[l..])); // Concatena o vetor de String com o resultado da chamada recursiva da função para um slice da String 
    sentences // Retorna o vetor de Strings
}

pub fn extract_stop_words(text: &str) -> std::collections::HashSet<String> {
    let mut stop_words = HashSet::new();
    for word in text.split_whitespace() {
        stop_words.insert(word.to_string().to_lowercase());
    }
    stop_words
}