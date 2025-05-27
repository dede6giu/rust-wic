struct ActorSWM {
    receiver: mpsc::Receiver<MessageSWM>,
    handle_wcc: ActorWCC,
    stop_words: HashSet<String>,
}

enum MessageSWM {
    Init {
        ref_wcm: ActorWCM,
    },
    Filter { sentence: String },
    Done, // Depois vemos
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
            MessageSWM::Init { ref_wcm, STOP_WORDS_FILE } => {
                // Lógica para criar o hashmap a partir do arquivo
            }
            MessageSWM::Filter { sentence } => {
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
            MessageSWM::Done => {
                self.handle_wcc.send(MessageWCC::Done).unwrap();
            }
        }
    }
}