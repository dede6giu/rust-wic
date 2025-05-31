use actix::prelude::*;
use crate::actors::WCM;
use crate::errors::keyword_add_error::KeywordAddError;
use crate::utils::text_processing::{extract_stop_words};
use crate::errors::{filter_error, reqwic_error};
use std::fs;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct ActorSWM {
    ref_wcm: Addr::<WCM::ActorWCM>,
    raw_stop_words: String,
    stop_words: HashSet<String>,
}
impl ActorSWM {
    pub fn new(child_wcm: Addr::<WCM::ActorWCM>) -> Self {
        ActorSWM { 
            ref_wcm: child_wcm,
            raw_stop_words: String::new(),
            stop_words: HashSet::new(),
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
#[rtype(result = "bool")]
pub struct Ping();
impl Handler<Ping> for ActorSWM {
    type Result = bool;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor {} ping!", "SWM");
        true
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
    pub fn new(path_stopwords: String) -> Self {
        Setup { 
            path_stopwords,
        }
    }
}
impl Handler<Setup> for ActorSWM {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Setup, _ctx: &mut Context<Self>) -> Self::Result {
        self.raw_stop_words = fs::read_to_string(msg.path_stopwords).expect("Não foi possível ler o arquivo");
        self.stop_words = extract_stop_words(&self.raw_stop_words);

        Ok(true)
    }
}

// ===== ReqWIC =====
// - Envia ReqWIC para WCM
#[derive(Message)]
#[rtype(result = "Result<HashMap<String, Vec<String>>, reqwic_error::ReqWICError>")]
pub struct ReqWIC { }
impl ReqWIC {
    pub fn new() -> Self {
        ReqWIC { }
    }
}
impl Handler<ReqWIC> for ActorSWM {
    type Result = ResponseFuture<Result<HashMap<String, Vec<String>>, reqwic_error::ReqWICError>>;

    fn handle(&mut self, _msg: ReqWIC, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {    
            let result_reqwic = this.ref_wcm
                .send(WCM::ReqWIC::new())
                .await
                .map_err(|_| reqwic_error::ReqWICError::SendError)? 
                .map_err(|_| reqwic_error::ReqWICError::WCMReqWICError)?;
            Ok(result_reqwic)
        })
    }
}


// ===== Filter =====
// - Recebe uma frase
// - Palavra por palavra, checa se é stopword
//     - Se sim, próxima iteração
//     - Se não, envia KeywordAdd para WCM
#[derive(Message)]
#[rtype(result = "Result<bool, filter_error::FilterError>")]
pub struct Filter {
    pub sentence: String,
}
impl Filter {
    pub fn new(sentence: String) -> Self {
        Filter {
            sentence: sentence,
        }
    }
}
impl Handler<Filter> for ActorSWM {
    type Result = ResponseFuture<Result<bool, filter_error::FilterError>>;

    fn handle(&mut self, msg: Filter, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();
        let shared_sentence = Arc::new(msg.sentence.clone());

        Box::pin(async move {
            for word in msg.sentence.split_whitespace() {
                if !this.stop_words.contains(&word.to_lowercase()) {
                    // println!("FILTER: {}", word);
                    let _ = this.ref_wcm
                            .send(WCM::KeywordAdd::new(word.to_string(), Arc::clone(&shared_sentence)))
                            .await;

                        /*
                        .map_err(|_| filter_error::FilterError::SendError)?
                        .map_err(|e| filter_error::FilterError::KeywordAddError(e))?;
                        */
                }
            }
            Ok(true)
        })
    }
}