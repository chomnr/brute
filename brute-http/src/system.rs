use std::sync::Arc;

use actix::{Actor, AsyncContext, Context, Handler, WrapFuture};
use ipinfo::IpInfo;
use log::info;
use reporter::BruteReporter;
use sqlx::{Pool, Postgres};

use crate::model::Individual;

pub trait Brute {}

//////////////////////
// SYSTEM /w ACTOR //
////////////////////

#[derive(Clone)]
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
        let brute_system = Arc::new(self.clone());
        BruteReporter::new(brute_system)
    }
}

impl Brute for BruteSystem {}

impl Actor for BruteSystem {
    type Context = Context<Self>;
}

impl Handler<Individual> for BruteSystem {
    type Result = ();

    fn handle(&mut self, msg: Individual, ctx: &mut Self::Context) -> Self::Result {
        let reporter = self.reporter();

        let fut = Box::pin(async move {
            reporter.start_report(msg).await;
            info!("Received a new attacker")
        });

        // Spawn the future as an actor message.
        ctx.spawn(fut.into_actor(self));
    }
}

///////////////
// REPORTER //
/////////////

pub mod reporter {
    use super::{Brute, BruteSystem};
    use crate::model::{Individual, ProcessedIndividual};
    use std::{
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };
    use uuid::Uuid;

    pub trait Reporter {}

    #[allow(async_fn_in_trait)]
    pub trait Reportable<T: Reporter, R> {
        async fn report(reporter: T, model: R) -> Option<Self>
        where
            Self: Sized;
    }

    #[derive(Clone)]
    pub struct BruteReporter<T: Brute> {
        brute: Arc<T>, // Use Arc to handle shared ownership
    }

    impl BruteReporter<BruteSystem> {
        pub fn new(brute: Arc<BruteSystem>) -> Self {
            BruteReporter { brute }
        }

        pub async fn start_report(&self, payload: Individual) {
            let individual = Individual::report(self.clone(), payload).await.unwrap();
            let processed_individual = ProcessedIndividual::report(self.clone(), individual)
                .await
                .unwrap();
        }
    }

    impl Reporter for BruteReporter<BruteSystem> {}

    ///////////
    // DATA //
    /////////

    // individual
    impl Reportable<BruteReporter<BruteSystem>, Individual> for Individual {
        async fn report(reporter: BruteReporter<BruteSystem>, mut model: Self) -> Option<Self> {
            let pool = &reporter.brute.db_pool;
            let query = r#"
                INSERT INTO individual (id, username, password, ip, protocol, timestamp)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#;
            model.id = Uuid::new_v4().as_simple().to_string();
            model.timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64;
            sqlx::query(query)
                .bind(&model.id())
                .bind(&model.username())
                .bind(&model.password())
                .bind(&model.ip())
                .bind(&model.protocol())
                .bind(model.timestamp())
                .execute(pool)
                .await
                .unwrap();
            Some(model)
        }
    }

    // processed individual
    impl Reportable<BruteReporter<BruteSystem>, Individual> for ProcessedIndividual {
        async fn report(
            reporter: BruteReporter<BruteSystem>,
            model: Individual,
        ) -> Option<ProcessedIndividual> {
            let query = r#"
                INSERT INTO processed_individual (
                id, username, password, ip, protocol, hostname, city, region, country, loc, org, postal,
                asn, asn_name, asn_domain, asn_route, asn_type,
                company_name, company_domain, company_type,
                vpn, proxy, tor, relay, hosting, service,
                abuse_address, abuse_country, abuse_email, abuse_name, abuse_network, abuse_phone,
                domain_ip, domain_total, domains, timestamp
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                    $13, $14, $15, $16, $17,
                    $18, $19, $20,
                    $21, $22, $23, $24, $25, $26,
                    $27, $28, $29, $30, $31, $32,
                    $33, $34, $35, $36
                );"#;
            // check if ip already exists
            // if ip already exists get the last_ip that was inserted
            // check the timestamp if the timestamp exceeds 5 minutes then
            // hit the ipinfo api again if not then just retrieve the data. (just helps with accuracy...).
            todo!()
        }
    }
}
