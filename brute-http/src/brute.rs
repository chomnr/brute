use std::sync::Arc;

use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use ipinfo::IpInfo;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use crate::attacker::{self, metric, AttackerRequest};

pub struct Brute {
    pub pool: Pool<Postgres>,
    pub ipinfo: IpInfo
}

impl Actor for Brute {
    type Context = Context<Self>;
}

impl Handler<AttackerRequest> for Brute {
    type Result = ();

    fn handle(&mut self, msg: AttackerRequest, ctx: &mut Self::Context) -> Self::Result {
        let pool = self.pool.clone();
        let fut = Box::pin(async move {
            // does the magic...
            attacker::metric::perform(msg.payload, pool).await;
            println!("Processed a request...")
        });
        let actor_future = fut.into_actor(self);
        ctx.spawn(actor_future);
    }
}

impl Brute {
    pub fn new(pool: Pool<Postgres>, ipinfo: IpInfo) -> Self {
        Self {
            pool,
            ipinfo
        }
    }
}