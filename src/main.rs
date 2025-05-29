mod actors {
    pub mod UT;
    pub mod WCC;
    pub mod DSM;
    pub mod SWM;
    pub mod WCM;
}
mod utils;

use actix::System;
use actix::Actor;
use crate::actors::WCC;
use crate::actors::DSM;
use crate::actors::SWM;
use crate::actors::WCM;

#[actix::main]
async fn main() {
    println!("Hello, world!");
    
    let path_input = String::new();
    let path_stopw = String::new();

    let actor_wcm = WCM::ActorWCM::new().start(); 
    let actor_swm = SWM::ActorSWM::new(actor_wcm).start();
    let actor_dsm = DSM::ActorDSM::new(actor_swm).start();
    let actor_wcc = WCC::ActorWCC::new(actor_dsm).start();

    let res = actor_wcc.send(WCC::Startup::new(path_input, path_stopw)).await.unwrap();

    match res {
        Ok(_) => println!("Sucesso!"),
        Err(_) => println!("Erro..."),
    }

    System::current().stop();
}
