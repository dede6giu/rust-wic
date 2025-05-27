use std::sync::mpsc;
use std::collections::HashMap;
use std::collections::HashSet;

struct ActorWCC {
    receiver: mpsc::Receiver<WCCMessage>,
    handle_dsm: ActorDSM,
}
enum WCCMessage {
    RunWIC {
        ref_dsm: ActorDSM,
    },
    Display {
        data_map: HashMap<String, Vec<String>>,
    },
}

struct ActorDSM {
    receiver: mpsc::Receiver<DSMMessage>,
    handle_swm: ActorSWM,
    data_raw: String,
    data_done: Vec<Vec<String>>,
}
enum DSMMessage {
    Init {
        path_file_input: String,
        ref_swm: ActorSWM,
    },
    SendKeys {
        author: ActorWCC,
    },
}

struct ActorSWM {
    receiver: mpsc::Receiver<SWMMessage>,
    handle_wcm: ActorWCM,
    data_swlist: HashSet<String>,
}
enum SWMMessage {
    Init {
        ref_wcm: ActorWCM,
    },
    Filter {
        keyword: String,
        phrase: String,
    },
}

struct ActorWCM {
    receiver: mpsc::Receiver<WCMMessage>,
    data_map: HashMap<String, Vec<String>>
}
enum WCMMessage {
    Keyword {
        keyword: String,
        phrase: String,
    },
    RequestWICMap {
        author: ActorWCC,
    },
}