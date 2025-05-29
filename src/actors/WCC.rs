use actix::prelude::*;
use crate::actors::DSM;

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

// ===== Startup =====
// - Envia "Startup" para DSM
// - Retorna quando WCM terminar seu Startup
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Startup {   
    pub path_input: String,
    pub path_stopwords: String,
}
impl Startup {
    pub fn new(
        path_input: String,
        path_stopwords: String,
    ) -> Self {
        Startup { 
            path_input,
            path_stopwords,
        }
    }
}
impl Handler<Startup> for ActorWCC {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(
        &mut self,
        msg: Startup,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();

        Box::pin(async move {    
            let res = this.ref_dsm.send(DSM::Startup::new(msg.path_input, msg.path_stopwords)).await.unwrap();
            res
        })
    }
}