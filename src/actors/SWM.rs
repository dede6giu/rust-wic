use actix::prelude::*;
use crate::actors::WCM;
use std::collections::HashSet;

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

// ===== Startup =====
// - Processa o stopwords e salva internamente
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Startup {
    pub path_stopwords: String,
}
impl Startup {
    pub fn new(
        path_stopwords: String,
    ) -> Self {
        Startup { 
            path_stopwords,
        }
    }
}
impl Handler<Startup> for ActorSWM {
    type Result = Result<bool, std::io::Error>;

    fn handle(
        &mut self,
        _msg: Startup,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();

        // TODO: Processamento de stopword

        Ok(true)
    }
}