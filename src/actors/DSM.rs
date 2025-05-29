use actix::prelude::*;
use crate::actors::SWM;

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

// ===== Startup =====
// - Envia "Startup" para SWM
// - Processa o input e salva internamente
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
impl Handler<Startup> for ActorDSM {
    type Result = ResponseFuture<Result<bool, std::io::Error>>;

    fn handle(
        &mut self,
        msg: Startup,
        _ctx: &mut Context<Self>
    ) -> Self::Result {
        let this = self.clone();

        // TODO: Processamento de input

        Box::pin(async move {    
            let res = this.ref_swm.send(SWM::Startup::new(msg.path_stopwords)).await.unwrap();
            res
        })
    }
}