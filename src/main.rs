mod actors {
    pub mod UT;
}

use actix::System;
use actix::Actor;
use crate::actors::UT;

#[actix::main]
async fn main() {
    println!("Hello, world!");
    
    let addr = UT::UTPing::new(0).start();
    let res = addr.send(UT::Ping(1)).await;

    println!("RESULT: {}", res.unwrap());

    System::current().stop();
}
