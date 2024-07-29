use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use sqlx::{Pool, Postgres};

use crate::attacker::AttackerRequest;

pub struct Brute {
    pub pool: Pool<Postgres>
}

impl Actor for Brute {
    type Context = Context<Self>;
}

impl Handler<AttackerRequest> for Brute {
    type Result = ();

    fn handle(&mut self, msg: AttackerRequest, ctx: &mut Self::Context) -> Self::Result {
        let fut = Box::pin(async {
            //sleep(Duration::from_secs(2)).await; // Simulating async work
            // do the database magic here...
            println!("DONE")
        });
        let actor_future = fut.into_actor(self);
        ctx.spawn(actor_future);
    }
}

impl Brute {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool
        }
    }
}