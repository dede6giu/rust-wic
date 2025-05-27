use std::sync::mpsc;
use std::collections::HashMap;

struct ActorWCC {
    receiver: mpsc::Receiver<MessageWCC>,
    handle_dsm: ActorDSM,
}

enum MessageWCC {
    RunWIC {
        ref_dsm: ActorDSM,
    },

    Display {
        data_map: HashMap<String, Vec<(String, String)>>,
    },
}

impl ActorWCC {
    fn new(
        receiver: mpsc::Receiver<MessageWCC>,
        handle_dsm: ActorDSM,
    ) -> Self {
        ActorWCC {
            receiver,
            handle_dsm,
        }
    }

    fn message_handle(&mut self, msg: MessageWCC) {
        match msg {
            // ===== RUNWIC =====
            // - Inicia o processo do Word In Context
            // - Salva referência para DSM para uso interno
            // - (opcionalmente) Requisita configurações extras 
            // - Envia PROCESSKEYS para DSM
            MessageWCC::RunWIC { ref_dsm } => {},

            // ===== RUNWIC =====
            // - Ordena data_map duplamente alfabeticamente
            // - Usa data_map para apresentar cada keyword
            //     - Usa cada keyword para apresentar todas suas 
            //       aparições (todos os itens da chave)
            // - Deve imprimir modelo: "{key} {frase adaptada} (from: {frase original})"
            MessageWCC::Display { data_map } => {},

            // Mensagem não identificada
            _ => {}
        }
    }
}