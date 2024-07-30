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
    use ipinfo::{AbuseDetails, AsnDetails, CompanyDetails, DomainsDetails, PrivacyDetails};
    use std::{
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };
    use uuid::Uuid;

    pub trait Reporter {}

    #[allow(async_fn_in_trait)]
    pub trait Reportable<T: Reporter, R> {
        async fn report(reporter: T, model: R) -> anyhow::Result<Self>
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
            let processed_individual = ProcessedIndividual::report(self.clone(), individual).await.unwrap();
        }
    }

    impl Reporter for BruteReporter<BruteSystem> {}

    ///////////
    // DATA //
    /////////

    // individual
    impl Reportable<BruteReporter<BruteSystem>, Individual> for Individual {
        async fn report(
            reporter: BruteReporter<BruteSystem>,
            mut model: Self,
        ) -> anyhow::Result<Self> {
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
                .await?;
            Ok(model)
        }
    }

    // processed individual

    impl Reportable<BruteReporter<BruteSystem>, Individual> for ProcessedIndividual {
        async fn report(
            reporter: BruteReporter<BruteSystem>,
            model: Individual,
        ) -> anyhow::Result<ProcessedIndividual> {
            // mistakes were made i probably could have just used the
            // structs that were given to me by ipinfo. it is what is.
            // you live you learn. chomnr 4:01 am florida 7/30/2024.
            let pool = &reporter.brute.db_pool;
            let ipinfo = &reporter.brute.ipinfo_client;
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as i64;

            let select_query = "SELECT *
                FROM processed_individual
                WHERE ip = $1
                ORDER BY timestamp DESC
                LIMIT 1;";

            let insert_query = r#"
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
                ) RETURNING *;"#;

            let asn_default = AsnDetails {
                asn: String::default(),
                name: String::default(),
                domain: String::default(),
                route: String::default(),
                asn_type: String::default(),
            };

            let company_default = CompanyDetails {
                name: String::default(),
                domain: String::default(),
                company_type: String::default(),
            };

            let abuse_default = AbuseDetails {
                address: String::default(),
                country: String::default(),
                email: String::default(),
                name: String::default(),
                network: String::default(),
                phone: String::default(),
            };

            let privacy_default = PrivacyDetails {
                vpn: false,
                proxy: false,
                tor: false,
                relay: false,
                hosting: false,
                service: String::default(),
            };

            let domain_default = DomainsDetails {
                ip: Some(String::default()),
                total: 0,
                domains: Vec::default(),
            };

            let ip_exists = sqlx::query_as::<_, ProcessedIndividual>(select_query)
                .bind(&model.ip())
                .fetch_optional(pool)
                .await
                .unwrap();

            let pi = if let Some(result) = ip_exists {
                if now - result.timestamp > 300_000 {
                    let mut ipinfo_lock = ipinfo.lock();
                    let ip_details = ipinfo_lock.lookup(&model.ip()).await?;

                    let asn_details = ip_details.asn.as_ref().unwrap_or(&asn_default);
                    let company_details = ip_details.company.as_ref().unwrap_or(&company_default);
                    let abuse_details = ip_details.abuse.as_ref().unwrap_or(&abuse_default);
                    let domain_details = ip_details.domains.as_ref().unwrap_or(&domain_default);
                    let privacy_details = ip_details.privacy.as_ref().unwrap_or(&privacy_default);

                    sqlx::query_as::<_, ProcessedIndividual>(insert_query)
                        .bind(&model.id())
                        .bind(&model.username())
                        .bind(&model.password())
                        .bind(&model.ip())
                        .bind(&model.protocol())
                        .bind(&ip_details.hostname)
                        .bind(&ip_details.city)
                        .bind(&ip_details.region)
                        .bind(&ip_details.country)
                        .bind(&ip_details.loc)
                        .bind(&ip_details.org)
                        .bind(&ip_details.postal)
                        .bind(&asn_details.asn)
                        .bind(&asn_details.name)
                        .bind(&asn_details.domain)
                        .bind(&asn_details.route)
                        .bind(&asn_details.asn_type)
                        .bind(&company_details.name)
                        .bind(&company_details.domain)
                        .bind(&company_details.company_type)
                        .bind(privacy_details.vpn)
                        .bind(privacy_details.proxy)
                        .bind(privacy_details.tor)
                        .bind(privacy_details.relay)
                        .bind(privacy_details.hosting)
                        .bind(&privacy_details.service)
                        .bind(&abuse_details.address)
                        .bind(&abuse_details.country)
                        .bind(&abuse_details.email)
                        .bind(&abuse_details.name)
                        .bind(&abuse_details.network)
                        .bind(&abuse_details.phone)
                        .bind(&domain_details.ip)
                        .bind(domain_details.total as i64)
                        .bind(&domain_details.domains)
                        .bind(model.timestamp)
                        .fetch_one(pool)
                        .await?
                } else {
                    sqlx::query_as::<_, ProcessedIndividual>(insert_query)
                        .bind(&model.id())
                        .bind(&model.username())
                        .bind(&model.password())
                        .bind(&model.ip())
                        .bind(&model.protocol())
                        .bind(&result.hostname())
                        .bind(&result.city())
                        .bind(&result.region())
                        .bind(&result.country())
                        .bind(&result.loc())
                        .bind(&result.org())
                        .bind(&result.postal())
                        .bind(&result.asn())
                        .bind(&result.asn_name())
                        .bind(&result.asn_domain())
                        .bind(&result.asn_route())
                        .bind(&result.asn_type())
                        .bind(&result.company_name())
                        .bind(&result.company_domain())
                        .bind(&result.company_type())
                        .bind(result.vpn())
                        .bind(result.proxy())
                        .bind(result.tor())
                        .bind(result.relay())
                        .bind(result.hosting())
                        .bind(&result.service())
                        .bind(&result.abuse_address())
                        .bind(&result.abuse_country())
                        .bind(&result.abuse_email())
                        .bind(&result.abuse_name())
                        .bind(&result.abuse_network())
                        .bind(&result.abuse_phone())
                        .bind(&result.domain_ip())
                        .bind(result.domain_total())
                        .bind(&result.domains())
                        .bind(model.timestamp)
                        .fetch_one(pool)
                        .await?
                }
            } else {
                let mut ipinfo_lock = ipinfo.lock();
                let ip_details = ipinfo_lock.lookup(&model.ip()).await.unwrap();

                let asn_details = ip_details.asn.as_ref().unwrap_or(&asn_default);
                let company_details = ip_details.company.as_ref().unwrap_or(&company_default);
                let abuse_details = ip_details.abuse.as_ref().unwrap_or(&abuse_default);
                let domain_details = ip_details.domains.as_ref().unwrap_or(&domain_default);
                let privacy_details = ip_details.privacy.as_ref().unwrap_or(&privacy_default);

                sqlx::query_as::<_, ProcessedIndividual>(insert_query)
                    .bind(&model.id())
                    .bind(&model.username())
                    .bind(&model.password())
                    .bind(&model.ip())
                    .bind(&model.protocol())
                    .bind(&ip_details.hostname)
                    .bind(&ip_details.city)
                    .bind(&ip_details.region)
                    .bind(&ip_details.country)
                    .bind(&ip_details.loc)
                    .bind(&ip_details.org)
                    .bind(&ip_details.postal)
                    .bind(&asn_details.asn)
                    .bind(&asn_details.name)
                    .bind(&asn_details.domain)
                    .bind(&asn_details.route)
                    .bind(&asn_details.asn_type)
                    .bind(&company_details.name)
                    .bind(&company_details.domain)
                    .bind(&company_details.company_type)
                    .bind(privacy_details.vpn)
                    .bind(privacy_details.proxy)
                    .bind(privacy_details.tor)
                    .bind(privacy_details.relay)
                    .bind(privacy_details.hosting)
                    .bind(&privacy_details.service)
                    .bind(&abuse_details.address)
                    .bind(&abuse_details.country)
                    .bind(&abuse_details.email)
                    .bind(&abuse_details.name)
                    .bind(&abuse_details.network)
                    .bind(&abuse_details.phone)
                    .bind(&domain_details.ip)
                    .bind(domain_details.total as i64)
                    .bind(&domain_details.domains)
                    .bind(model.timestamp)
                    .fetch_one(pool)
                    .await?
            };
            Ok(pi)
        }
    }
}
