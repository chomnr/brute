use actix::Message;
use serde::Deserialize;

use crate::{flags::Flags, model::Individual};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct AttackerRequest {
    pub payload: Individual,
    flags: Flags
}

impl Message for AttackerRequest {
    type Result = ();
}

impl AttackerRequest {
    pub fn new(payload: Individual, flags: Flags) -> Self {
        Self {
            payload,
            flags,
        }
    }
}
 

// metrics...
pub mod metric {
    use std::sync::{Arc, MutexGuard};

    use actix::Addr;
    use axum::http::StatusCode;
    use sqlx::{Pool, Postgres};
    use tokio::sync::Mutex;

    use crate::model::Individual;

    pub async fn perform(individual: Individual, pool: Pool<Postgres>) {
        todo!()
    }

    // individual here..
    fn create_individual(indivdual: Individual) -> anyhow::Result<(), (StatusCode, String)> {
        todo!()
    }

    //insert individual
    //insert processed_individual
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