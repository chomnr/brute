use actix::prelude::*;
use std::time::Duration;
use tokio::time::sleep;
use serde::Deserialize;
use sqlx::{Pool, Postgres};

use crate::flags::Flags;

/*
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Attacker {
    /// PAM_USER
    username: String,

    /// PAM_AUTHTOK
    password: String,

    /// PAM_RHOST
    ip: String,

    /// PAM_SERVICE
    protocol: String,
}

#[derive(Debug, Clone)]
pub struct AttackerRequest {
    /// attacker's payload
    payload: Attacker,

    /// the flags that we want to run.
    flags: Flags,

    /// postgres pool.
    pool: Pool<Postgres>
}

impl Message for AttackerRequest {
    type Result = ();
}

impl Actor for Attacker {
    type Context = Context<Self>;
}

impl Handler<AttackerRequest> for Attacker {
    type Result = ();

    fn handle(&mut self, msg: AttackerRequest, ctx: &mut Self::Context) -> Self::Result {
        // Spawn a new async task to handle the request
        let fut = Box::pin(async {
            sleep(Duration::from_secs(2)).await; // Simulating async work
            println!("DONE")
        });
        let actor_future = fut.into_actor(self);
        ctx.spawn(actor_future);
    }
}
*/

/* 
impl Actor for AttackerRequest {
    type Msg = AttackerRequest;

    async fn recv(
        &mut self,
        ctx: &riker::actor::Context<Self::Msg>,
        msg: Self::Msg,
        sender: riker::actor::Sender,
    ) {
        let pool = self.pool.clone();
        println!("Received: {:#?}", msg);
        // process database load here...
    }
}*/

/* 
#[derive(Default, Debug, Clone, Deserialize)]
pub struct Attacker {
    /// PAM_USER
    username: String,

    /// PAM_AUTHTOK
    password: String,

    /// PAM_RHOST
    ip: String,

    /// PAM_SERVICE
    protocol: String,
}

#[derive(Debug, Clone)]
pub struct AttackerRequest {
    /// attacker's payload
    payload: Attacker,

    /// the flags that we want to run.
    flags: Flags,
}

impl AttackerRequest {
    pub fn new(payload: Attacker, flags: Flags) -> Self {
        Self {
            payload,
            flags,
        }
    }

    pub fn new_r(username: &str, password: &str, ip: &str, protocol: &str, flags: Flags) -> Self {
        Self {
            payload: Attacker {
                username: String::from(username),
                password: String::from(password),
                ip: String::from(ip),
                protocol: String::from(protocol),
            },
            flags,
        }
    }
}
*/

/*
impl Actor for Attacker {
    type Msg = AttackerRequest;

    fn recv(
        &mut self,
        ctx: &riker::actor::Context<Self::Msg>,
        msg: Self::Msg,
        sender: riker::actor::Sender,
    ) {
        println!("Received: {:#?}", msg)
        // process database load here...
    }
}
*/