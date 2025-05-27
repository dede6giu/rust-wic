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
    fn new(receiver: mpsc::Receiver<MessageDSM>, handle_swm: ActorSWM,  FILE_PATH: String) -> Self {
        ActorDSM {
            receiver,
            handle_smw,
            read_file(FILE_PATH),
            sentences,
        }
    }
    fn message_handle(&mut self, msg: MessageDSM) {
        match msg {
            MessageDSM::Init { path_file_input, ref_swm } => {
                let content = fs::read_to_string(path_file_input).unwrap();
                let sentences: Vec<&str> = content.split('.').collect(); // Exemplo simplista
                for sentence in sentences {
                    ref_swm.send(MessageSWM::Filter {
                        sentence: sentence.trim().to_string(),
                    }).unwrap();
                }
                ref_swm.send(MessageSWM::Done).unwrap();
            }
            MessageDSM::SendKeys { author } => {},
            _ => {}
        }
    }
}