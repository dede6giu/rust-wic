use actix::prelude::*;
use std::fs;
use crate::errors::sendkeys_error::SendkeysError;
use crate::utils::text_processing::{extract_sentences};
use crate::actors::SWM;
// use crate::actors::WCM;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ActorDSM {
    ref_swm: Addr::<SWM::ActorSWM>,
    data_raw: String,
    sentences: Vec<String>,
}
impl ActorDSM {
    pub fn new(child_swm: Addr::<SWM::ActorSWM>) -> Self {
        ActorDSM { 
            ref_swm: child_swm,
            data_raw: String::new(),
            sentences: Vec::new()
        }
    }
}
impl Actor for ActorDSM {
    type Context = Context<Self>;
}

// ===== Ping =====
// - Verifica se o ator está funcionando.
// - Imprime mensagem no console.
#[derive(Message)]
#[rtype(result = "bool")]
pub struct Ping();
impl Handler<Ping> for ActorDSM {
    type Result = bool;

    fn handle(&mut self, _msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        println!("Actor {} ping!", "DSM");
        true
    }
}

// ===== Setup =====
// - Envia "Setup" para SWM
// - Processa o input e salva internamente
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
impl Handler<Setup> for ActorDSM {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: Setup, _ctx: &mut Context<Self>) -> Self::Result {
        self.data_raw = fs::read_to_string(msg.path_input).expect("Não foi possível ler o arquivo");
        self.sentences = extract_sentences(&self.data_raw);
        let this = self.clone();
        Box::pin(async move {
            this.ref_swm.send(SWM::Setup::new(msg.path_stopwords)).await.unwrap()
        })
    }
}

// ===== SendKeys =====
// - Envia cada frase para SWM (mensagem Filter)
// - Espera SWM terminar seu próprio processo
// - Envia Transmit para SWM (ReqWIC para WCM)
// - Retorna HashMap pronto
#[derive(Message)]
#[rtype(result = "Result<HashMap<String, Vec<String>>, SendkeysError>")]
pub struct SendKeys { }
impl SendKeys {
    pub fn new() -> Self {
        SendKeys { }
    }
}
impl Handler<SendKeys> for ActorDSM {
    type Result = ResponseFuture<Result<HashMap<String, Vec<String>>, SendkeysError>>;

    fn handle(&mut self, _msg: SendKeys, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {
            for sentence in this.sentences {
                this.ref_swm
                    .send(SWM::Filter::new(sentence)) 
                    .await
                    .map_err(|_| SendkeysError::SendError)?
                    .map_err(|e| SendkeysError::FilterError(e))?;
            }
            
            this.ref_swm
                .send(SWM::ReqWIC::new())
                .await
                .map_err(|_| SendkeysError::SendError)? 
                .map_err(|e| SendkeysError::ReqWICError(e))
        })
    }
}