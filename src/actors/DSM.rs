use actix::prelude::*;
use std::fs;
use crate::utils::text_processing::{self, extract_sentences};
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
    pub fn new(
        child_swm: Addr::<SWM::ActorSWM>,
    ) -> Self {
        ActorDSM { 
            ref_swm: child_swm,
            data_raw: String::new(),
            sentences: Vec::new(),
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
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping();
impl Handler<Ping> for ActorDSM {
    type Result = Result<bool, std::io::Error>;

    fn handle(
        &mut self,
        _msg: Ping,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        println!("Actor {} ping!", "DSM");
        Ok(true)
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
    pub fn new(
        path_input: String,
        path_stopwords: String,
    ) -> Self {
        Setup { 
            path_input,
            path_stopwords,
        }
    }
}
impl Handler<Setup> for ActorDSM {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(&mut self, msg: Setup, _ctx: &mut Context<Self>) -> Self::Result {
        // `this` é uma cópia de `self`. Não depende, portanto, do seu lifetime, que é limitado, pois a posse de `self` só foi emprestada ao escopo da função.
        let mut this = self.clone();

        // Lê o arquivo
        this.data_raw = fs::read_to_string(msg.path_input).expect("Não foi possível ler o arquivo");
        // Forma o vetor de Strings
        this.sentences = text_processing::extract_sentences(&self.data_raw);


        // O `async move` cria uma espécie de struct (o Future) em que os atributos são cada variável contida em seu corpo (os valores das variáveis são movidos para esses atributos)
        // Por isso que precisamos do this, não poderíamos fazer o move de um valor emprestado (self)
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
#[rtype(result = "Result<HashMap<String, Vec<String>>, std::io::Error>")]
pub struct SendKeys { }
impl SendKeys {
    pub fn new() -> Self {
        SendKeys { }
    }
}
impl Handler<SendKeys> for ActorDSM {
    type Result = ResponseFuture<Result<HashMap<String, Vec<String>>, std::io::Error>>;

    fn handle(
        &mut self,
        _msg: SendKeys,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {
            // TODO
            // Para cada frase:
            // - Envie Filter(phrase)
            // - Esperar Result
            // - Verificar Ok()
            // - Próxima iteração
            this.ref_swm.send(SWM::Filter::new()).await.unwrap();
            
            // TODO
            // - Enviar Transmit(WCM::ReqWIC) para SWM
            this.ref_swm.send(SWM::ReqWIC::new()).await.unwrap()
        })
    }
}