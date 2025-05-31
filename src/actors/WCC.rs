use actix::prelude::*;
use crate::actors::DSM;
use crate::errors::dsm_error::DSMError;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ActorWCC {
    ref_dsm: Addr::<DSM::ActorDSM>,
}
impl ActorWCC {
    pub fn new(child_dsm: Addr::<DSM::ActorDSM>) -> Self {
        ActorWCC { 
            ref_dsm: child_dsm,
        }
    }
}
impl Actor for ActorWCC {
    type Context = Context<Self>;
}

// ===== Ping =====
// - Verifica se o ator está funcionando.
// - Imprime mensagem no console.
#[derive(Message)]
#[rtype(result = "bool")]
pub struct Ping();
impl Handler<Ping> for ActorWCC {
    type Result = bool;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor {} ping!", "WCC");
        true
    }
}

// ===== Setup =====
// - Envia "Setup" para DSM
// - Retorna quando WCM terminar seu Setup
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Setup {   
    pub path_input: String,
    pub path_stopwords: String,
}
impl Setup {
    pub fn new(path_input: String, path_stopwords: String) -> Self {
        Setup { 
            path_input,
            path_stopwords,
        }
    }
}
impl Handler<Setup> for ActorWCC {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: Setup, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {    
            this.ref_dsm
            .send(DSM::Setup::new(msg.path_input, msg.path_stopwords))
            .await
            .unwrap()
        })
    }
}

// ===== Run =====
// - Envia "SendKeys" para DSM
// - Retorna HashMap pronto
#[derive(Message)]
#[rtype(result = "Result<HashMap<String, Vec<String>>, DSMError>")]
pub struct Run { }
impl Run {
    pub fn new() -> Self {
        Run { }
    }
}
impl Handler<Run> for ActorWCC {
    type Result = ResponseFuture<Result<HashMap<String, Vec<String>>, DSMError>>;

    fn handle(&mut self, _msg: Run, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {
            this.ref_dsm
            .send(DSM::SendKeys::new())
            .await
            .unwrap()
        })
    }
}


// ===== Display =====
// - Envia "SendKeys" para DSM
// - Retorna HashMap pronto
#[derive(Message)]
#[rtype(result = "Result<bool, DSMError>")]
pub struct Display {
    words_context: HashMap<String, Vec<String>>
}
impl Display {
    pub fn new(words_context: HashMap<String, Vec<String>>) -> Self {
        Display {
            words_context,
        }
    }
}
impl Handler<Display> for ActorWCC {
    type Result = Result<bool, DSMError>;

    fn handle(&mut self, msg: Display, _ctx: &mut Context<Self>) -> Self::Result {
        let pre_keys = msg.words_context.keys();
        let mut keys: Vec<String> = vec![];
        for key in pre_keys {
            keys.push(key.to_string());
        }
        keys.sort_unstable();
        for key in keys {
            let mut results = msg.words_context.get(&key).unwrap().to_vec();
            results.sort_unstable();
            for sentence in results {
                println!("{}", sentence);
            }
        }
        
        Ok(true)
    }
}