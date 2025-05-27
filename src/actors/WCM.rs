use std::sync::mpsc;
use std::collections::HashMap;

struct ActorWCM {
    receiver: mpsc::Receiver<MessageWCM>,
    data_map: HashMap<String, Vec<(String, String)>>,
    size_context: u64,
}

enum MessageWCM {
    Keyword {
        keyword: String,
        phrase: String,
    },

    RequestWICMap {
        author: Actor,
    },

    Ping,
}

impl ActorWCM {
    fn new( receiver: mspc::Receiver<MessageWCM> ) -> Self {
        ActorWCM { 
            receiver, 
            data_map: HashMap<String, Vec<String>>::new(), 
            size_context: 2, // Atualmente hardcoded
        }
    }

    fn message_handle(&mut self, msg: ActorMessage) {
        match msg {
            // ===== KEYWORD =====
            // - Recolhe palavras antes e depois de acordo com 
            //   o tamanho de size_context
            // - Adiciona item phrase na lista da chave keyword
            //     - keyword: [(og_phrase, ordered_phrase)]
            MessageWCM::Keyword { keyword, phrase } => {},

            // ===== REQUESTWICMAP =====
            // - Envia para author a mensagem DISPLAY
            MessageWCM::RequestWICMap { author } => {}

            // ===== PING =====
            // - Verifica funcionalidade do Ator
            MessageWCM::Ping => {},

            // Mensagem não identificada
            _ => {}
        }
    }
}