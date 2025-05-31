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