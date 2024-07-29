use actix::Message;
use serde::Deserialize;

use crate::flag::Flags;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Attacker {
    username: String,
    password: String,
    ip: String,
    protocol: String,
}

impl Attacker {
    pub fn new(username: &str, password: &str, ip: &str, protocol: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
            ip: String::from(ip),
            protocol: String::from(protocol)
        }
    }
}

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AttackerRequest {
    payload: Attacker,
    flags: Flags
}

impl Message for AttackerRequest {
    type Result = ();
}

impl AttackerRequest {
    pub fn new(payload: Attacker, flags: Flags) -> Self {
        Self {
            payload,
            flags,
        }
    }
}
 

// metrics...
mod metric {
    use sqlx::{Pool, Postgres};

    use super::Attacker;

    fn insert_attacker(payload: Attacker) {

    }
}
/*
pub mod attacker_sql {
    use actix::Addr;
    use sqlx::{Pool, Postgres};

    use crate::brute::Brute;

    use super::Attacker;

    /// create and store attacker in db
    fn create_and_store_attacker(payload: Attacker, pool: Pool<Postgres>) {
        // store metrics...
        // return result then progogate it to actor..

        //
    }
}
*/