use std::collections::HashSet;

pub fn extract_sentences(text: &str) -> Vec<String> {
    let mut sentence = match text.chars().next() {
        Some(c) => c.to_string(), 
        None => return vec![], 
    };
    
    let mut l = 1;
    for letter in text[1..].chars() {
        if !letter.is_uppercase() {
            sentence.push(letter); 
            l += 1;
        } else {
            break;
        }
    };

    let mut sentences = vec![sentence.trim().to_string()];
    sentences.extend(extract_sentences(&text[l..]));
    sentences 
}

pub fn extract_stop_words(text: &str) -> std::collections::HashSet<String> {
    let mut stop_words = HashSet::new();
    for word in text.split_whitespace() {
        stop_words.insert(word.to_string().to_lowercase());
    }
    stop_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sentences_basico_com_divisao() {
        // Entrada: "umaFraseSimples"
        // Esperado: ["umaFrase", "Simples"]
        assert_eq!(extract_sentences("umaFraseSimples"), vec!["uma", "Frase", "Simples"]);
    }

    #[test]
    fn stopwords_basico_com_conversao_para_minusculas() {
        // Testa a funcionalidade central de separar palavras por espaço,
        // convertê-las para minúsculas e inseri-las em um HashSet.
        let mut esperado = HashSet::new();
        esperado.insert("palavra1".to_string());
        esperado.insert("palavra2".to_string());
        assert_eq!(extract_stop_words("Palavra1 PALAVRA2"), esperado);
    }
}
