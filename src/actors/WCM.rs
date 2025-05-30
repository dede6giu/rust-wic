use actix::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
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
    sentence: Arc<String>,
}
impl KeywordAdd {
    pub fn new(key: String, sentence: Arc<String>) -> Self {
        KeywordAdd {
            key,
            sentence,
        }
    }
}
impl Handler<KeywordAdd> for ActorWCM {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: KeywordAdd, _ctx: &mut Context<Self>) -> Self::Result {
        let SIZE_WIC_AROUND = 2; // Transformar em não-hardcoded eventualmente
        let parts = msg.sentence.split(" ");
        let word_list = parts.collect::<Vec<&str>>();

        let mut i_pos = 0;
        for (i, word) in word_list.iter().enumerate() {
            // TODO: cheque sempre lowercase
            if *word == msg.key {
                i_pos = i;
            }
        }

        let key = msg.key;
        let value = String::new();
        if word_list.len() <= SIZE_WIC_AROUND*2 + 1 {
            let mut result = "".to_owned();
            result.push_str(&key);
            result.push_str(" ");
            for word in word_list.iter().skip(i_pos) {
                
            }
        } else {

        }


        let this = self.clone();
        Box::pin(async move {
            let wic_dict = this.word_context.lock().await;
            wic_dict;
        });

        Ok(true)
    }
}