use actix::prelude::*;
use std::fs;
use crate::utils::text_processing::{extract_sentences};
use crate::errors::{filter_error, dsm_error, reqwic_error};
use crate::actors::SWM;
use crate::actors::WCM;
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
// Trait obtigatório para qualquer ator no actix
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
        // Lê o arquivo
        self.data_raw = fs::read_to_string(msg.path_input).expect("Não foi possível ler o arquivo");
        // Forma o vetor de Strings
        self.sentences = extract_sentences(&self.data_raw);

        // `this` é uma cópia de `self`. Logo, o this, tem posse sobre seus valores, diferente do self, que só tem um empréstimo dessa posse, o que nos impede de mover seus valores nesse escopo (em que só temos um empréstimo dos valores em self)
        let this = self.clone();


        // O `async move` cria uma espécie de struct (o Future) em que os atributos são cada variável contida em seu corpo (os valores das variáveis são movidos para esses atributos)
        // Por isso que precisamos do this. Não poderíamos fazer o move de um valor emprestado (self)
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
#[rtype(result = "Result<HashMap<String, Vec<String>>, dsm_error::DSMError>")]
pub struct SendKeys { }
impl SendKeys {
    pub fn new() -> Self {
        SendKeys { }
    }
}
impl Handler<SendKeys> for ActorDSM {
    type Result = ResponseFuture<Result<HashMap<String, Vec<String>>, dsm_error::DSMError>>;

    fn handle(&mut self, _msg: SendKeys, _ctx: &mut Context<Self>) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {
            // Note que não percorremos `senteces` por referência, o que seria o padrão para não mover os elementos do vetor (normalmente, não é desejável) ou gerar cópias (o que é custoso)
            for sentence in this.sentences {
                let result_filter = this.ref_swm
                .send(SWM::Filter::new(sentence)) // Percorrer por referência (com for sentence in &this.sentences) nos obrigaria a enviar `sentence.clone()` para o Filter, pois não poderíamos enviar um valor emprestado no escopo do for (teríamos problema de lifetime). Isso seria mais custoso do que fazer o `move` desses elementos, o que só seria um problema se precisássemos utilizá-los novamente (não precisaremos)
                .await
                .map_err(|_| dsm_error::DSMError::FilterSendError(filter_error::FilterError::SendError))?; // Usamos o map_err para transformar o erro retornado pelo Filter (FilterError) em uma variante do DSMErrror (FilterSendError)

                result_filter.map_err(|e| dsm_error::DSMError::FilterSendError(e))?;
            }
            
            // TODO
            // - Enviar Transmit(WCM::ReqWIC) para SWM
            let result_reqwic = this.ref_swm
                .send(SWM::ReqWIC::new())
                .await
                .map_err(|_| dsm_error::DSMError::ReqWICSendError(reqwic_error::ReqWICError::SendError))?; // Usamos o map_err para transformar o erro retornado pelo ReqWIC (ReqWICError) em uma variante do DSMErrror (ReqWICSendError)

            result_reqwic.map_err(dsm_error::DSMError::ReqWICSendError)
        })
    }
}