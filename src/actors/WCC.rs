use actix::prelude::*;
use crate::actors::DSM;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ActorWCC {
    ref_dsm: Addr::<DSM::ActorDSM>,
}
impl ActorWCC {
    pub fn new(
        child_dsm: Addr::<DSM::ActorDSM>,
    ) -> Self {
        ActorWCC { 
            ref_dsm: child_dsm,
        }
    }
}
impl Actor for ActorWCC {
    type Context = Context<Self>;
}

// ===== Ping =====
// - Verifica se o ator está funcionando.
// - Imprime mensagem no console.
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping();
impl Handler<Ping> for ActorWCC {
    type Result = Result<bool, std::io::Error>;

    fn handle(
        &mut self,
        _msg: Ping,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        println!("Actor {} ping!", "WCC");
        Ok(true)
    }
}

// ===== Setup =====
// - Envia "Setup" para DSM
// - Retorna quando WCM terminar seu Setup
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
impl Handler<Setup> for ActorWCC {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(
        &mut self,
        msg: Setup,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {    
            let res = this.ref_dsm.send(DSM::Setup::new(msg.path_input, msg.path_stopwords)).await.unwrap();
            res
        })
    }
}

// ===== Run =====
// - Envia "SendKeys" para DSM
// - Retorna HashMap pronto
#[derive(Message)]
#[rtype(result = "Result<HashMap<String, Vec<(String, String)>>, std::io::Error>")]
pub struct Run {
    
}
impl Run {
    pub fn new(
        
    ) -> Self {
        Run { 
            
        }
    }
}
impl Handler<Run> for ActorWCC {
    type Result = Result<HashMap<String, Vec<String>>, std::io::Error>;

    fn handle(
        &mut self,
        _msg: Run,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {    
            let res = this.ref_dsm.send(DSM::SendKeys::new()).await.unwrap();
            res
        })
    }
}