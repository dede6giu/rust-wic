use std::sync::mpsc;

struct ActorDSM {
    receiver: mpsc::Receiver<MessageDSM>,
    handle_swm: ActorSWM,
    data_raw: String,
    sentences: Vec<String>,
}

enum MessageDSM {
    Init {
        path_file_input: String,
        ref_swm: ActorSWM,
    },

    SendKeys {
        author: ActorWCC
    },
}

impl ActorDSM {
    fn new(receiver: mpsc::Receiver<MessageDSM>, handle_swm: ActorSWM) -> Self {
        ActorDSM {
            receiver,
            handle_swm,
            data_raw: String::new(),
            sentences: Vec<String>::new(),
        }
    }
    
    fn message_handle(&mut self, msg: MessageDSM) {
        match msg {
            // ===== INIT =====
            // - Salva parâmetro ref_swm dentro do struct para uso
            // - Abre o arquivo path_file_input e salva na interidade como string em data_raw
            // - Separa data_raw em frases, salve em sentences
            MessageDSM::Init { ref_swm, path_file_input } => {
                // Implementação exemplo
                let content = fs::read_to_string(path_file_input).unwrap();
                let sentences: Vec<&str> = content.split('.').collect();
                for sentence in sentences {
                    ref_swm.send(MessageSWM::Filter {
                        sentence: sentence.trim().to_string(),
                    }).unwrap();
                }
                ref_swm.send(MessageSWM::Done).unwrap();
            }

            // ===== PROCESSKEYS =====
            // - Envia cada frase individualmente para SWM pela mensagem FILTER
            // - Envia PROCESSDONE para SWM quando atingir o término
            MessageDSM::ProcessKeys { author: ActorWCC } => {},

            // Mensagem não identificada
            _ => {}
        }
    }
}