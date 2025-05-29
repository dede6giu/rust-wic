use actix::prelude::*;
use crate::actors::WCM;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct ActorSWM {
    ref_wcm: Addr::<WCM::ActorWCM>,
    data_raw: String,
    list_phrase: HashSet<String>,

}
impl ActorSWM {
    pub fn new(
        child_wcm: Addr::<WCM::ActorWCM>,
    ) -> Self {
        ActorSWM { 
            ref_wcm: child_wcm,
            data_raw: String::new(),
            list_phrase: HashSet::new(),
        }
    }
}
impl Actor for ActorSWM {
    type Context = Context<Self>;
}

// ===== Ping =====
// - Verifica se o ator está funcionando.
// - Imprime mensagem no console.
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping();
impl Handler<Ping> for ActorSWM {
    type Result = Result<bool, std::io::Error>;

    fn handle(
        &mut self,
        _msg: Ping,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        println!("Actor {} ping!", "SWM");
        Ok(true)
    }
}

// ===== Setup =====
// - Processa o arquivo stopwords e salva internamente
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Setup {
    pub path_stopwords: String,
}
impl Setup {
    pub fn new(
        path_stopwords: String,
    ) -> Self {
        Setup { 
            path_stopwords,
        }
    }
}
impl Handler<Setup> for ActorSWM {
    type Result = Result<bool, std::io::Error>;

    fn handle(
        &mut self,
        _msg: Setup,
        _ctx: &mut Context<Self>
    ) -> Self::Result {

        // TODO: Processamento de stopword

        Ok(true)
    }
}

// ===== Transmit =====
// - Envia uma mensagem para baixo (WCM)
#[derive(Message)]
#[rtype(result = "Result<HashMap<String, Vec<String>>, std::io::Error>")]
pub struct Transmit<T: actix::Message> {
    dwn_msg: T,
}
impl<T: actix::Message> Transmit<T> {
    pub fn new(
        dwn_msg: T,
    ) -> Self {
        Transmit {
            dwn_msg,
        }
    }
}
impl<T: actix::Message> Handler<Transmit<T>> for ActorSWM {
    type Result = ResponseFuture<Result<HashMap<String, Vec<String>>, std::io::Error>>;

    fn handle(
        &mut self,
        _msg: Transmit<T>,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        
    }
}