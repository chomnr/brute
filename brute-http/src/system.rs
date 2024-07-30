use std::sync::Arc;

use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use ipinfo::IpInfo;
use reporter::BruteReporter;
use sqlx::{Pool, Postgres};

use crate::model::Individual;

pub trait Brute {}

//////////////////////
// SYSTEM /w ACTOR //
////////////////////

pub struct BruteSystem {
    /// PostgreSQL connection pool.
    pub db_pool: Pool<Postgres>,

    /// IP info client with shared access.
    pub ipinfo_client: Arc<parking_lot::Mutex<IpInfo>>,
}

impl BruteSystem {
    /// Creates a new instance of `BruteSystem`.
    ///
    /// # Panics
    ///
    /// Panics if the provided database pool is closed.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Create the PostgreSQL connection pool
    /// let brute_config = BruteConfig::default();
    ///
    /// // Create an instance of BruteSystem
    /// let brute_system = BruteSystem::new(brute_config); // as an actor you will append .start() at the end.s
    /// ```
    pub async fn new_brute(pg_pool: Pool<Postgres>, ipinfo_client: IpInfo) -> Self {
        Self {
            db_pool: pg_pool,
            ipinfo_client: Arc::new(parking_lot::Mutex::new(ipinfo_client)),
        }
    }

    /// Reports data to the database.
    pub fn reporter(&self) -> BruteReporter<BruteSystem> {
        // Clone the BruteSystem if possible or wrap it in RefCell
        BruteReporter::new(self)
    }
}

impl Brute for BruteSystem {}

impl Actor for BruteSystem {
    type Context = Context<Self>;
}

impl Handler<Individual> for BruteSystem {
    type Result = ();

    fn handle(&mut self, msg: Individual, ctx: &mut Self::Context) -> Self::Result {
        //let pool = self.db_pool.clone();
        //let ipinfo = self.ipinfo_client.clone();
        let reporter = self.reporter();
        let fut = Box::pin(async move { println!("requested recieved") });

        ctx.spawn(fut.into_actor(self));
    }
}

///////////////
// REPORTER //
/////////////

pub mod reporter {
    use crate::model::Individual;

    use super::{Brute, BruteSystem};

    pub trait Reporter {}

    pub trait Reportable<T: Reporter> {
        async fn handle(reporter: T, model: Self) -> Self;
    }

    #[derive(Clone)]
    pub struct BruteReporter<'a, T: Brute> {
        brute: &'a T,
    }

    impl<'a> BruteReporter<'a, BruteSystem> {
        pub fn new(brute: &'a BruteSystem) -> Self {
            BruteReporter { brute }
        }

        pub async fn start_report(self, individual: Individual) {
            Individual::handle(self, individual).await;
        }
    }

    impl<'a> Reporter for BruteReporter<'a, BruteSystem> {}

    ///////////
    // DATA //
    /////////
    
    impl<'a> Reportable<BruteReporter<'a, BruteSystem>> for Individual {
        async fn handle(reporter: BruteReporter<'a, BruteSystem>, model: Self) -> Self {
            let pool = &reporter.brute.db_pool;
            // do the queryness..
            todo!()
        }
    }
    
}
