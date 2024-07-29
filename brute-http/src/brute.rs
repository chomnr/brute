use std::{env::var, sync::Arc};

use actix::{Actor, AsyncContext, Context, Handler, Message, WrapFuture};
use anyhow::{anyhow, bail, Result};
use axum::http::StatusCode;
use ipinfo::IpInfo;
use parking_lot::Mutex;
use sqlx::{Pool, Postgres};

use crate::{error::BruteError, model::Individual};

////////////////////
// CONFIGURATION //
//////////////////

/// Configuration for the `Brute` service.
pub struct BruteConfig {
    /// Database connection string.
    pub conn_string: String,

    /// API token for IP info service.
    pub ipinfo_token: String,
}

impl BruteConfig {
    /// # Panics
    ///
    /// Panics if the connection string is empty or does not start with "postgresql://",
    /// or if the IP info access token is empty.
    ///
    /// ```ignore
    /// let config = BruteConfig::new("postgresql://{username}:{password}@{host}/{database}", "some_otoken");
    /// ```
    fn new(conn_string: &str, ipinfo_token: &str) -> Self {
        if conn_string.is_empty() {
            panic!("The connection string cannot be empty.")
        }

        if !conn_string.starts_with("postgresql://") {
            panic!("The connection string must start with 'postgresql://'")
        }

        if ipinfo_token.is_empty() {
            panic!("The ipinfo access token cannot be empty.")
        }

        BruteConfig {
            conn_string: String::from(conn_string),
            ipinfo_token: String::from(ipinfo_token),
        }
    }
}

impl Default for BruteConfig {
    fn default() -> Self {
        Self {
            conn_string: var("BRUTE_DATABASE_URL").unwrap(),
            ipinfo_token: var("BRUTE_IPINFO_TOKEN").unwrap(),
        }
    }
}

//////////////////////
// SYSTEM /w ACTOR //
////////////////////

pub struct BruteSystem {
    /// PostgreSQL connection pool.
    pub db_pool: Pool<Postgres>,

    /// IP info client with shared access.
    pub ipinfo_client: Arc<Mutex<IpInfo>>,
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
    /// let database_pool = PgPoolOptions::new()
    ///     .connect("postgresql://username:password@localhost/database")
    ///     .await
    ///     .expect("Failed to create database pool");
    ///
    /// // Create the IP info configuration
    /// let ipinfo_config = IpInfoConfig {
    ///     token: Some(ipinfo_access_token),
    ///     ..Default::default()
    /// };
    ///
    /// // Initialize the IP info client
    /// let ipinfo = IpInfo::new(ipinfo_config).expect("Failed to initialize IpInfo");
    ///
    /// // Create an instance of BruteSystem
    /// let brute_system = BruteSystem::new(database_pool, Arc::new(Mutex::new(ipinfo))); // as an actor you will append .start() at the end.s
    /// ```
    pub fn new(database_pool: Pool<Postgres>, ipinfo_client: Arc<Mutex<IpInfo>>) -> Self {
        if database_pool.is_closed() {
            panic!("The database pool is closed.")
        }

        Self {
            db_pool: database_pool,
            ipinfo_client,
        }
    }
}

impl Actor for BruteSystem {
    type Context = Context<Self>;
}

impl Handler<Request<Individual>> for BruteSystem {
    type Result = anyhow::Result<StatusCode, BruteError>;

    fn handle(&mut self, msg: Request<Individual>, ctx: &mut Self::Context) -> Self::Result {
        let pool = self.db_pool.clone();
        let ipinfo = self.ipinfo_client.clone();

        let fut = Box::pin(async move { println!("requested recieved") });

        ctx.spawn(fut.into_actor(self));
        Ok(StatusCode::OK)
    }
}

////////////////////////
// REQUEST FOR ACTOR //
//////////////////////

pub struct Request<T> {
    payload: T,
}

static MAX_USERNAME_LENGTH: usize = 32;
static MAX_PASSWORD_LENGTH: usize = 128;
static MAX_IP_LENGTH: usize = 15;

static MIN_USERNAME_LENGTH: usize = 1;
static MIN_PASSWORD_LENGTH: usize = 1;
static MIN_IP_LENGTH: usize = 7;

impl Request<Individual> {
    /// Creates a new `Request` instance with validation checks.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the fields fail to meet the specified length requirements:
    /// - `400 Bad Request` if the username or password is too short or too long,
    ///   or if the IP address is too short.
    ///
    /// # Examples
    ///
    /// ```
    /// let individual = Individual {
    ///     username: "user".to_string(),
    ///     password: "pass".to_string(),
    ///     ip: "192.168.1.1".to_string(),
    /// };
    /// let request = Request::new(individual);
    /// ```
    pub fn new(payload: Individual) -> Result<Self, BruteError> {
        let (username, password, ip) = (&payload.username, &payload.password, &payload.ip);

        if username.len() > MAX_USERNAME_LENGTH {
            return Err(BruteError {
                status_code: 400,
                message: "Username exceeds maximum length.".to_string(),
            });
        }

        if username.len() < MIN_USERNAME_LENGTH {
            return Err(BruteError {
                status_code: 400,
                message: "Username is too short.".to_string(),
            });
        }

        if password.len() > MAX_PASSWORD_LENGTH {
            return Err(BruteError {
                status_code: 400,
                message: "Password exceeds maximum length.".to_string(),
            });
        }

        if password.len() < MIN_PASSWORD_LENGTH {
            return Err(BruteError {
                status_code: 400,
                message: "Password is too short.".to_string(),
            });
        }

        if ip.len() > MAX_IP_LENGTH {
            return Err(BruteError {
                status_code: 400,
                message: "IP address is too short.".to_string(),
            });
        }

        // Optional: Check IP length if needed
        if ip.len() < MIN_IP_LENGTH {
            return Err(BruteError {
                status_code: 400,
                message: "IP address is too short.".to_string(),
            });
        }

        Ok(Request { payload })
    }

    pub fn new_request(
        username: &str,
        password: &str,
        ip: &str,
        protocol: &str,
    ) -> Result<Self, BruteError> {
        let request = Self::new(Individual {
            username: String::from(username),
            password: String::from(password),
            ip: String::from(ip),
            protocol: String::from(protocol),
        })?;
        Ok(Request {
            payload: request.payload,
        })
    }
}

impl Message for Request<Individual> {
    type Result = anyhow::Result<StatusCode, BruteError>;
}