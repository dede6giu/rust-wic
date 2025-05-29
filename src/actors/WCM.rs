use actix::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ActorWCM {
    words_context: HashMap<String, Vec<(String, String)>>
}
impl ActorWCM {
    pub fn new(
        
    ) -> Self {
        ActorWCM { 
            words_context: HashMap::new(),
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