use actix::prelude::*;

pub struct UTPing {
    count: usize,
}

impl Actor for UTPing {
    type Context = Context<Self>;
}

impl UTPing {
    pub fn new(st_count: usize) -> Self {
        UTPing { 
            count: st_count,
        }
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Ping(pub usize);

impl Handler<Ping> for UTPing {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}