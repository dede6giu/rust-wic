use actix::prelude::*;
use std::fs;
use crate::utils::text_processing::{self, extract_senteces};
use crate::actors::SWM;

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

    fn handle(&mut self, msg: Startup, _ctx: &mut Context<Self>) -> Self::Result {
        // `this` é uma cópia de `self`. Não depende, portanto, do seu lifetime, que é limitado, pois a posse de `self` só foi emprestada ao escopo da função.
        let mut this = self.clone();

        // Lê o arquivo
        this.data_raw = fs::read_to_string(msg.path_input).expect("Não foi possível ler o arquivo");
        // Forma o vetor de Strings
        this.sentences = text_processing::extract_senteces(&self.data_raw);


        // O `async move` cria uma espécie de struct (o Future) em que os atributos são cada variável contida em seu corpo (os valores das variáveis são movidos para esses atributos)
        // Por isso que precisamos do this, não poderíamos fazer o move de um valor emprestado (self)
        Box::pin(async move {    
            this.ref_swm.send(SWM::Startup::new(msg.path_stopwords)).await.unwrap()
        })
    }
}