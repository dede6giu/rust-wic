use actix::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::utils::text_processing::make_circular_sentence;

type Db = HashMap<String, Vec<String>>;

#[derive(Clone)]
pub struct ActorWCM {
    word_in_context: Arc<Mutex<Db>>, 
}
impl ActorWCM {
    pub fn new() -> Self {
        ActorWCM {
            word_in_context: Arc::new(Mutex::new(Db::new())),
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
#[rtype(result = "bool")]
pub struct Ping();
impl Handler<Ping> for ActorWCM {
    type Result = bool;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor {} ping!", "WCM");
        true
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

    fn handle(&mut self, _msg: ReqWIC, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();
        
        Box::pin(async move {
            let guard = this.word_in_context
            .lock()
            .await;

            Ok(guard.clone())
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
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: KeywordAdd, _ctx: &mut Context<Self>) -> Self::Result {
        let words: Vec<&str> = msg.sentence.split_whitespace().collect();
        let key = msg.key.to_lowercase();

        let circular_sentence = make_circular_sentence(&key, &words);
        let this = self.clone();
        Box::pin(async move {
            let mut wic_dict = this.word_in_context.lock().await;
            if wic_dict.contains_key(&key) {
                let mut new_value: Vec<String> = wic_dict.get(&key).unwrap().to_vec();
                new_value.push(circular_sentence.to_string());
                
                // debug
                // for deb in new_value.iter() {
                //     println!("{}: {}", key, deb);
                // }

                wic_dict.insert(key, new_value);

            } else {
                let mut new_value: Vec<String> = Vec::new();
                new_value.push(circular_sentence.to_string());
                
                // debug
                // for deb in new_value.iter() {
                //     println!("{}: {}", key, deb);
                // }

                wic_dict.insert(key, new_value);
            }
            
            Ok(true)
        })
    }
}