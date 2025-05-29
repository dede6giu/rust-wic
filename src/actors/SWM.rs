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

// ===== ReqWIC =====
// - Envia ReqWIC para WCM
#[derive(Message)]
#[rtype(result = "Result<HashMap<String, Vec<String>>, std::io::Error>")]
pub struct ReqWIC { }
impl ReqWIC {
    pub fn new() -> Self {
        ReqWIC { }
    }
}
impl Handler<ReqWIC> for ActorSWM {
    type Result = ResponseFuture<Result<HashMap<String, Vec<String>>, std::io::Error>>;

    fn handle(
        &mut self,
        _msg: ReqWIC,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        
        let this = self.clone();

        // TODO: Processamento de input

        Box::pin(async move {    
            this.ref_wcm.send(WCM::ReqWIC::new()).await.unwrap()
        })

    }
}


// ===== Filter =====
// - Recebe uma frase
// - Palavra por palavra, checa se é stopword
//     - Se sim, próxima iteração
//     - Se não, envia KeywordAdd para WCM
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Filter { }
impl Filter {
    pub fn new() -> Self {
        Filter { }
    }
}
impl Handler<Filter> for ActorSWM {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(
        &mut self,
        _msg: Filter,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();
        
        Box::pin(async move {
            // TODO: Processamento de input
            let res = this.ref_wcm.send(WCM::KeywordAdd::new(/* keyword */, /* phrase */)).await.unwrap();
            res
        })
    }
}