use actix::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

type Db = HashMap<String, Vec<String>>;

#[derive(Clone)]
pub struct ActorWCM {
    word_context: Arc<Mutex<Db>>, 
}
impl ActorWCM {
    pub fn new() -> Self {
        ActorWCM {
            word_context: Arc::new(Mutex::new(Db::new())),
        }
    }
}
impl Actor for ActorWCM {
    type Context = Context<Self>;
}

// ===== Ping =====
// - Verifica se o ator está funcionando.
// - Imprime mensagem no console.
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping();
impl Handler<Ping> for ActorWCM {
    type Result = Result<bool, std::io::Error>;

    fn handle(
        &mut self,
        _msg: Ping,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        println!("Actor {} ping!", "WCM");
        Ok(true)
    }
}

// ===== ReqWIC =====
// - Envia o HashMap atual.
#[derive(Message)]
#[rtype(result = "Result<Db, std::io::Error>")]
pub struct ReqWIC { }
impl ReqWIC {
    pub fn new() -> Self {
        ReqWIC { }
    }
}
impl Handler<ReqWIC> for ActorWCM {
    type Result = ResponseFuture<Result<Db, std::io::Error>>;

    fn handle(
        &mut self,
        _msg: ReqWIC,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();
        
        Box::pin(async move {
            let wic_dict = this.word_context.lock().await.clone();
            Ok(wic_dict)
        })
    }
}


// ===== KeywordAdd =====
// - Adiciona uma nova frase em keyword
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct KeywordAdd {
    key: String,
    phrase: String,
}
impl KeywordAdd {
    pub fn new(
        key: String,
        phrase: String,
    ) -> Self {
        KeywordAdd {
            key,
            phrase,
        }
    }
}
impl Handler<KeywordAdd> for ActorWCM {
    type Result = Result<bool, std::io::Error>;

    fn handle(
        &mut self,
        _msg: KeywordAdd,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        // TODO: Método para adicionar palavra no dicionário
        Ok(true)
    }
}