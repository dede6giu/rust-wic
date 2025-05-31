mod actors {
    pub mod UT;
    pub mod WCC;
    pub mod DSM;
    pub mod SWM;
    pub mod WCM;
}
mod utils;
mod errors;

use std::collections::HashMap;

use actix::System;
use actix::Actor;
use crate::actors::WCC;
use crate::actors::DSM;
use crate::actors::SWM;
use crate::actors::WCM;

#[actix::main]
async fn main() {

    println!("Iniciando...");
    let actor_wcm = WCM::ActorWCM::new().start(); 
    let actor_swm = SWM::ActorSWM::new(actor_wcm).start();
    let actor_dsm = DSM::ActorDSM::new(actor_swm).start();
    let actor_wcc = WCC::ActorWCC::new(actor_dsm).start();
    
    
    // TODO
    // Requisita paths para os arquivos de input e stopword
    // Requisita path para output
    let path_input = "data/input.txt".to_string();
    let path_stopw = "data/stopwords.txt".to_string();
    let path_outpt = String::new();


    // Inicia cadeia de mensagens de Setup
    let res_setup = actor_wcc.send(WCC::Setup::new(path_input, path_stopw)).await.unwrap();
    match res_setup {
        Ok(_) => { /* println!("Sucesso no Setup!") */ },
        Err(e) => println!("Erro no Setup: {}", e),
    }


    // Inicia o programa
    let res_run = actor_wcc.send(WCC::Run::new()).await.unwrap();
    let mut hash_res: HashMap<String, Vec<String>> = HashMap::new();
    match res_run {
        Ok(res_hash) => {
            // println!("Sucesso no Processamento!");
            hash_res = res_hash;
        },
        Err(_) => println!("Erro no Processamento."),
    }

    // Imprime o resultado
    let res_display = actor_wcc.send(WCC::Display::new(hash_res)).await.unwrap();
    match res_display {
        Ok(_) => { /* println!("Sucesso no Print!") */ },
        Err(_) => println!("Erro no Processamento."),
    }

    System::current().stop();

}