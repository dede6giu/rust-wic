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

pub fn make_circular_sentence(key: &str, words: &Vec<&str>) -> String {
    let SIZE_WIC_AROUND: usize = 2; // Quantidade de palavras desejadas após a keyword

    // Encontra a posição da keyword (case-insensitive)
    let keyword_pos = words.iter()
        .position(|w| w.to_lowercase() == key)
        .unwrap_or(0);

    // Calcula índices seguros para o contexto anterior
    let start_index = keyword_pos.saturating_sub(SIZE_WIC_AROUND);
    let before;

    if keyword_pos - start_index == SIZE_WIC_AROUND {
        // Caso simples: palavras suficientes antes da keyword
        before = words[start_index..keyword_pos].join(" ");
    } else {
        let size = words.len();
        let mut circular_before = Vec::new();

        if keyword_pos - start_index > 0 {
            circular_before.extend_from_slice(&words[start_index..keyword_pos]);
        }

        let additional_needed = SIZE_WIC_AROUND - (keyword_pos - start_index);

        let avaiable_from_end = (size - (keyword_pos + SIZE_WIC_AROUND + 1)).saturating_sub(0); // Conta todas as palavras disponíveis no final do vetor, retirando as que são consumidas pela extensão após a keyword

        let take_from_end = additional_needed.min(avaiable_from_end);

        if take_from_end > 0 {
            circular_before.extend_from_slice(&words[size - take_from_end..]);
        }

        before = circular_before.join(" ");
    }

    // Calcula quantas palavras reais existem após a keyword
    let available_after = words.len().saturating_sub(keyword_pos + 1);
    let after;

    if available_after >= SIZE_WIC_AROUND {
        // Caso normal: palavras suficientes após a keyword
        after = words[keyword_pos + 1..keyword_pos + 1 + SIZE_WIC_AROUND].join(" ");
    } else {
        // Lógica circular: completa com palavras do início
        let mut circular_after = Vec::new();
        
        // 1. Adiciona palavras reais após a keyword (se existirem)
        if available_after > 0 {
            circular_after.extend_from_slice(&words[keyword_pos + 1..]);
        }
        
        // 2. Calcula quantas palavras adicionais são necessárias
        let additional_needed = SIZE_WIC_AROUND - available_after;
        
        // 3. Pega palavras do início, evitando as já usadas no contexto anterior
        let available_from_start = start_index.saturating_sub(0); // Todas as palavras antes de start_index
        let take_from_start = additional_needed.min(available_from_start); // Pega o mínimo entre as palavras adicionais necessárias e as disponíveis no início, que são as primeiras palavras da string que já não foram pegas
        
        // 4. Adiciona palavras do início (se disponíveis)
        if take_from_start > 0 {
            circular_after.extend_from_slice(&words[..take_from_start]);
        }
        
        after = circular_after.join(" "); // Cria a String com o vetor de &str
    }

    // Formata o valor final
    let keyword = words[keyword_pos];

    format!(
        "{} {} {}",
        keyword,
        after,
        before
    )
}