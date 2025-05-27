use std::sync::mpsc;
use std::collections::HashMap;
use std::collections::HashSet;

enum Actor {
    ActorWCC,
    ActorWCM,
    ActorDSM,
    ActorSWM,
}
enum ActorMessage {
    MessageWCC,
    MessageWCM,
    MessageDSM,
    MessageSWM,
}

struct ActorWCC {
    receiver: mpsc::Receiver<MessageWCC>,
    handle_dsm: ActorDSM,
}
enum MessageWCC {
    RunWIC {
        ref_dsm: ActorDSM,
    },
    Display {
        data_map: HashMap<String, Vec<String>>,
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

    fn message_handle(&mut self, msg: ActorMessage) {
        match msg {
            MessageWCC::RunWIC { ref_dsm } => {},
            MessageWCC::Display { data_map } => {},
            _ => {},
        }
    }
}

struct ActorDSM {
    receiver: mpsc::Receiver<MessageDSM>,
    handle_swm: ActorSWM,
    data_raw: String,
    data_done: Vec<Vec<String>>,
}
enum MessageDSM {
    Init {
        path_file_input: String,
        ref_swm: ActorSWM,
    },
    SendKeys {
        author: Actor,
    },
}
impl ActorDSM {
    fn new(
        receiver: mpsc::Receiver<MessageDSM>,
        handle_swm: ActorSWM,
    ) -> Self {
        ActorDSM {
            receiver,
            handle_swm,
            data_raw: String::new(),
            data_done: Vec::new(),
        }
    }

    fn message_handle(&mut self, msg: ActorMessage) {
        match msg {
            MessageDSM::Init { path_file_input, ref_swm } => {},
            MessageDSM::SendKeys { author } => {},
            _ => {},
        }
    }
}

struct ActorSWM {
    receiver: mpsc::Receiver<MessageSWM>,
    handle_wcm: ActorWCM,
    data_swlist: HashSet<String>,
}
enum MessageSWM {
    Init {
        ref_wcm: ActorWCM,
    },
    Filter {
        keyword: String,
        phrase: String,
    },
}
impl ActorSWM {
    fn new(
        receiver: mpsc::Receiver<MessageSWM>,
        handle_wcm: ActorWCM,
    ) -> Self {
        ActorSWM {
            receiver,
            handle_wcm,
            data_swlist: Vec::new(),
        }
    }

    fn message_handle(&mut self, msg: ActorMessage) {
        match msg {
            MessageSWM::Init { ref_wcm } => {},
            MessageSWM::Filter { keyword, phrase } => {},
            _ => {},
        }
    }
}

struct ActorWCM {
    receiver: mpsc::Receiver<MessageWCM>,
    data_map: HashMap<String, Vec<String>>
}
enum MessageWCM {
    Keyword {
        keyword: String,
        phrase: String,
    },
    RequestWICMap {
        author: Actor,
    },
}
impl ActorWCM {
    fn new( receiver: mspc::Receiver<MessageWCM> ) -> Self {
        ActorWCM { 
            receiver, 
            data_map: HashMap<String, Vec<String>>::new(), 
        }
    }
    fn message_handle(&mut self, msg: ActorMessage) {
        match msg {
            MessageWCM::Keyword { keyword, phrase } => {},
            MessageWCM::RequestWICMap { author } => {}
            _ => {},
        }
    }
}

async fn startup_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

fn main() {

}