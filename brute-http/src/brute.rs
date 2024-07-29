use std::sync::Arc;

use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use ipinfo::IpInfo;
use sqlx::{Pool, Postgres};
use tokio::sync::Mutex;

use crate::attacker::{self, metric, AttackerRequest};

////////////
// ACTOR //
//////////

/// Represents an instance of Brute.
///
/// Contains the database connection pool and the IpInfo API client.
pub struct Brute {
    pub pool: Pool<Postgres>,
    pub ipinfo: Arc<Mutex<IpInfo>>
}

impl Actor for Brute {
    type Context = Context<Self>;
}

impl Handler<AttackerRequest> for Brute {
    type Result = ();

    fn handle(&mut self, msg: AttackerRequest, ctx: &mut Self::Context) -> Self::Result {
        let pg_pool= self.pool.clone();
        let ipinfo_instance = Arc::clone(&self.ipinfo);
        let fut = Box::pin(async move {
            attacker::metric::perform(msg.payload, pg_pool, ipinfo_instance).await;
            println!("Processed a request...")
        });
        let actor_future = fut.into_actor(self);
        ctx.spawn(actor_future);
    }
}

impl Brute {
    pub fn new(pool: Pool<Postgres>, ipinfo: Arc<Mutex<IpInfo>>) -> Self {
        Self {
            pool,
            ipinfo
        }
    }
}

//////////////
// REQUEST //
////////////