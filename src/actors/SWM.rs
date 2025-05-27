use std::sync::mpsc;
use std::collections::HashSet;

struct ActorSWM {
    receiver: mpsc::Receiver<MessageSWM>,
    handle_wcc: ActorWCC,
    stop_words: HashSet<String>,
}

enum MessageSWM {
    Init {
        ref_wcm: ActorWCM,
    },

    Filter { 
        sentence: String, 
    },

    ProcessDone {
        author: ActorWCC,
    },

    Ping,
}

impl ActorSWM {
    fn new(
        receiver: mpsc::Receiver<MessageSWM>,
        handle_wcm: ActorWCM,
    ) -> Self {
        ActorSWM {
            receiver,
            handle_wcm,
            HashSet<String>::new(),
        }
    }

    fn message_handle(&mut self, msg: MessageSWM) {
        match msg {
            // ===== Init =====
            // - Salva parâmetro handle_wcm dentro do struct para uso
            // - Abre o arquivo STOP_WORDS_FILE e salva cada palavra (em minúsculo) em stop_words
            MessageSWM::Init { ref_wcm, STOP_WORDS_FILE } => {}

            // ===== Filter =====
            // - Recebe uma frase e envia todas as possibilidades para
            //   WCM via KeywordEntry, EXCETO se estiver em stop_words
            MessageSWM::Filter { sentence } => {
                // Implementação exemplo
                let words: Vec<&str> = sentence.split_whitespace().collect();
                for word in words {
                    let word_lower = word.to_lowercase();
                    if !self.stop_words.contains(&word_lower) {
                        self.handle_wcc.send(MessageWCC::AddOccurrence {
                            word: word.to_string(),
                            sentence: sentence.clone(),
                        }).unwrap();
                    }
                }
            }

            // ==== ProcessDone =====
            // - Apenas transmite RequestWICMap para WCM
            MessageSWM::ProcessDone { author } => {}

            // ===== Ping =====
            // - Verifica funcionalidade do Ator
            MessageSWM::Ping => {},

            // Mensagem não identificada
            _ => {}
        }
    }
}