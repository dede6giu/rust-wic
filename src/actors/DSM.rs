use actix::prelude::*;
use crate::actors::SWM;
use crate::actors::WCM;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ActorDSM {
    ref_swm: Addr::<SWM::ActorSWM>,
    data_raw: String,
    list_phrase: Vec<String>,
}
impl ActorDSM {
    pub fn new(
        child_swm: Addr::<SWM::ActorSWM>,
    ) -> Self {
        ActorDSM { 
            ref_swm: child_swm,
            data_raw: String::new(),
            list_phrase: Vec::new(),
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

    fn handle(
        &mut self,
        msg: Setup,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();

        // TODO: Processamento de input

        Box::pin(async move {    
            let res = this.ref_swm.send(SWM::Setup::new(msg.path_stopwords)).await.unwrap();
            res
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