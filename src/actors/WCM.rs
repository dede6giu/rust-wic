use actix::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

type Db = HashMap<String, Vec<String>>;

static mut ST_WIC: LazyLock<Arc<Mutex<Db>>> = LazyLock::new(|| Arc::new( Mutex::new(HashMap::new())));

#[derive(Clone)]
pub struct ActorWCM {

}
impl ActorWCM {
    pub fn new(
        
    ) -> Self {
        ActorWCM {

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